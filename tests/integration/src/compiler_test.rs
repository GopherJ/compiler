#![allow(dead_code)]

use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::sync::Arc;

use miden_assembly::Assembler;
use miden_assembly::AssemblyContext;
use miden_codegen_masm::MasmCompiler;
use miden_diagnostics::term::termcolor::ColorChoice;
use miden_diagnostics::CodeMap;
use miden_diagnostics::DefaultEmitter;
use miden_diagnostics::DiagnosticsConfig;
use miden_diagnostics::DiagnosticsHandler;
use miden_diagnostics::Emitter;
use miden_diagnostics::NullEmitter;
use miden_diagnostics::SourceSpan;
use miden_diagnostics::Verbosity;
use miden_frontend_wasm::translate_module;
use miden_frontend_wasm::WasmTranslationConfig;

use miden_hir::pass::AnalysisManager;
use miden_hir::pass::RewritePass;
use miden_hir::pass::RewriteSet;
use miden_hir::FunctionIdent;
use miden_hir::Ident;
use miden_hir::ModuleRewritePassAdapter;
use miden_hir::ProgramBuilder;
use miden_hir::Symbol;
use miden_stdlib::StdLibrary;
use midenc_session::InputFile;
use midenc_session::Session;

pub enum CompilerTestSource {
    Rust(String),
    RustCargo {
        cargo_project_folder_name: String,
        artifact_name: String,
    },
    // Wasm(String),
    // Ir(String),
}

impl CompilerTestSource {
    pub fn artifact_name(&self) -> String {
        match self {
            CompilerTestSource::RustCargo {
                cargo_project_folder_name: _,
                artifact_name,
            } => artifact_name.clone(),
            _ => panic!("Not a Rust Cargo project"),
        }
    }
}

/// Compile to different stages (e.g. Wasm, IR, MASM) and compare the results against expected output
pub struct CompilerTest {
    /// The compiler session
    pub session: Session,
    /// The source code used to compile the test
    pub source: CompilerTestSource,
    /// The entrypoint function to use when building the IR
    entrypoint: Option<FunctionIdent>,
    /// The compiled Wasm component/module
    pub wasm_bytes: Vec<u8>,
    /// The compiled IR
    pub hir: Option<Box<miden_hir::Program>>,
    /// The compiled MASM
    pub ir_masm: Option<Arc<miden_codegen_masm::Program>>,
}

impl CompilerTest {
    /// Compile the Wasm component from a Rust Cargo project using cargo-component
    pub fn rust_source_cargo_component(cargo_project_folder: &str) -> Self {
        let manifest_path = format!("../rust-apps-wasm/{}/Cargo.toml", cargo_project_folder);
        // dbg!(&pwd);
        let mut cargo_build_cmd = Command::new("cargo");
        // Enable Wasm bulk-memory proposal (uses Wasm `memory.copy` op instead of `memcpy` import)
        cargo_build_cmd.env("RUSTFLAGS", "-C target-feature=+bulk-memory");
        cargo_build_cmd
            .arg("component")
            .arg("build")
            .arg("--manifest-path")
            .arg(manifest_path)
            .arg("--release")
            // compile std as part of crate graph compilation
            // https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
            .arg("-Z")
            .arg("build-std=std,core,alloc,panic_abort")
            .arg("-Z")
            // abort on panic without message formatting (core::fmt uses call_indirect)
            .arg("build-std-features=panic_immediate_abort");
        let mut child = cargo_build_cmd
            .arg("--message-format=json-render-diagnostics")
            .stdout(Stdio::piped())
            .spawn()
            .expect(
                format!(
                    "Failed to execute cargo build {}.",
                    cargo_build_cmd
                        .get_args()
                        .map(|arg| format!("'{}'", arg.to_str().unwrap()))
                        .collect::<Vec<_>>()
                        .join(" ")
                )
                .as_str(),
            );
        let reader = std::io::BufReader::new(child.stdout.take().unwrap());
        let mut wasm_artifacts = Vec::new();
        for message in cargo_metadata::Message::parse_stream(reader) {
            match message.expect("Failed to parse cargo metadata") {
                cargo_metadata::Message::CompilerArtifact(artifact) => {
                    // find the Wasm artifact in artifact.filenames
                    for filename in artifact.filenames {
                        if filename.as_str().ends_with(".wasm") {
                            wasm_artifacts.push(filename.into_std_path_buf());
                        }
                    }
                }
                _ => (),
            }
        }
        let output = child.wait().expect("Couldn't get cargo's exit status");
        if !output.success() {
            eprintln!("pwd: {:?}", std::env::current_dir().unwrap());
            let mut stderr = Vec::new();
            child
                .stderr
                .unwrap()
                .read(&mut stderr)
                .expect("Failed to read stderr");
            let stderr = String::from_utf8(stderr).expect("Failed to parse stderr");
            eprintln!("stderr: {}", stderr);
            panic!("Rust to Wasm compilation failed!");
        }
        assert!(output.success());
        assert_eq!(wasm_artifacts.len(), 1, "Expected one Wasm artifact");
        let wasm_comp_path = &wasm_artifacts.first().unwrap();
        let artifact_name = wasm_comp_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        Self {
            session: default_session(),
            source: CompilerTestSource::RustCargo {
                cargo_project_folder_name: cargo_project_folder.to_string(),
                artifact_name,
            },
            entrypoint: None,
            wasm_bytes: fs::read(wasm_artifacts.first().unwrap()).unwrap(),
            hir: None,
            ir_masm: None,
        }
    }

    /// Set the Rust source code to compile using a Cargo project and binary bundle name
    pub fn rust_source_cargo(
        cargo_project_folder: &str,
        artifact_name: &str,
        entrypoint: &str,
    ) -> Self {
        let manifest_path = format!("../rust-apps-wasm/{}/Cargo.toml", cargo_project_folder);
        // dbg!(&pwd);
        let temp_dir = std::env::temp_dir();
        let target_dir = temp_dir.join(cargo_project_folder);
        let output = Command::new("cargo")
            .arg("build")
            .arg("--manifest-path")
            .arg(manifest_path)
            .arg("--release")
            // .arg("--bins")
            .arg("--target=wasm32-unknown-unknown")
            // .arg("--features=wasm-target")
            .arg("--target-dir")
            .arg(target_dir.clone())
            // compile std as part of crate graph compilation
            // https://doc.rust-lang.org/cargo/reference/unstable.html#build-std
            .arg("-Z")
            .arg("build-std=core,alloc")
            .arg("-Z")
            // abort on panic without message formatting (core::fmt uses call_indirect)
            .arg("build-std-features=panic_immediate_abort")
            .output()
            .expect("Failed to execute cargo build.");
        if !output.status.success() {
            eprintln!("pwd: {:?}", std::env::current_dir().unwrap());
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            panic!("Rust to Wasm compilation failed!");
        }
        let target_bin_file_path = Path::new(&target_dir)
            .join("wasm32-unknown-unknown")
            .join("release")
            .join(artifact_name)
            .with_extension("wasm");
        // dbg!(&target_bin_file_path);
        let mut target_bin_file = fs::File::open(target_bin_file_path).unwrap();
        let mut wasm_bytes = vec![];
        Read::read_to_end(&mut target_bin_file, &mut wasm_bytes).unwrap();
        fs::remove_dir_all(target_dir).unwrap();

        let session = default_session();
        let entrypoint = FunctionIdent {
            module: Ident::new(Symbol::intern("noname"), SourceSpan::default()),
            function: Ident::new(
                Symbol::intern(entrypoint.to_string()),
                SourceSpan::default(),
            ),
        };
        CompilerTest {
            session,
            source: CompilerTestSource::RustCargo {
                cargo_project_folder_name: cargo_project_folder.to_string(),
                artifact_name: artifact_name.to_string(),
            },
            wasm_bytes,
            entrypoint: Some(entrypoint),
            hir: None,
            ir_masm: None,
        }
    }

    /// Set the Rust source code to compile
    pub fn rust_source_program(rust_source: &str) -> Self {
        let wasm_bytes = compile_rust_file(rust_source);
        let session = default_session();
        CompilerTest {
            session,
            source: CompilerTestSource::Rust(rust_source.to_string()),
            wasm_bytes,
            entrypoint: None,
            hir: None,
            ir_masm: None,
        }
    }

    /// Set the Rust source code to compile and add a binary operation test
    pub fn rust_fn_body(rust_source: &str) -> Self {
        let rust_source = format!(
            r#"
            #![no_std]
            #![no_main]

            #[panic_handler]
            fn my_panic(_info: &core::panic::PanicInfo) -> ! {{
                loop {{}}
            }}

            #[no_mangle]
            pub extern "C" fn entrypoint{}
            "#,
            rust_source
        );
        let wasm_bytes = compile_rust_file(&rust_source);
        let session = default_session();
        let entrypoint = FunctionIdent {
            module: Ident {
                name: Symbol::intern("noname"),
                span: SourceSpan::default(),
            },
            function: Ident {
                name: Symbol::intern("entrypoint"),
                span: SourceSpan::default(),
            },
        };

        CompilerTest {
            session,
            source: CompilerTestSource::Rust(rust_source.to_string()),
            wasm_bytes,
            entrypoint: Some(entrypoint),
            hir: None,
            ir_masm: None,
        }
    }

    /// Compare the compiled Wasm against the expected output
    pub fn expect_wasm(&self, expected_wat_file: expect_test::ExpectFile) {
        let wasm_bytes = self.wasm_bytes.as_ref();
        let wat = demangle(&wasm_to_wat(wasm_bytes));
        expected_wat_file.assert_eq(&wat);
    }

    /// Compare the compiled IR against the expected output
    pub fn expect_ir(&mut self, expected_hir_file: expect_test::ExpectFile) {
        let hir_program = if let Some(hir) = self.hir.as_ref() {
            hir
        } else {
            let hir_module = wasm_to_ir(&self.wasm_bytes, &self.session);
            let mut builder = ProgramBuilder::new(&self.session.diagnostics)
                .with_module(hir_module.into())
                .unwrap();
            if let Some(entrypoint) = self.entrypoint.as_ref() {
                builder = builder.with_entrypoint(entrypoint.clone());
            }
            let hir_program = builder.link().expect("Failed to link IR program");
            self.hir = Some(hir_program);
            self.hir.as_ref().unwrap()
        };
        // Program does not implement pretty printer yet, use the first module
        let ir_module = demangle(
            &hir_program
                .modules()
                .iter()
                .take(1)
                .collect::<Vec<&miden_hir::Module>>()
                .first()
                .expect("no module in IR program")
                .to_string()
                .as_str(),
        );
        expected_hir_file.assert_eq(&ir_module);
    }

    /// Compare the compiled MASM against the expected output
    pub fn expect_masm(&mut self, expected_masm_file: expect_test::ExpectFile) {
        let program = self.ir_masm_program();
        expected_masm_file.assert_eq(&program.to_string());
    }

    /// Get the compiled MASM as [`miden_assembly::Program`]
    pub fn vm_masm_program(&mut self) -> miden_core::Program {
        let assembler = Assembler::default()
            .with_library(&StdLibrary::default())
            .expect("Failed to load stdlib");
        let program = self.ir_masm_program();
        // TODO: get code map from the self.diagnostics
        let codemap = CodeMap::new();
        let program_ast = program.to_program_ast(&codemap);
        for module in program.modules() {
            let core_module = module.to_module_ast(&codemap);
            let _ = assembler
                .compile_module(
                    &core_module.ast,
                    Some(&core_module.path),
                    &mut AssemblyContext::for_module(false),
                )
                .expect(
                    format!(
                        "VM Assembler failed to compile module:\n{:?}\n with error",
                        core_module.ast
                    )
                    .as_str(),
                );
        }
        let core_program = assembler.compile_ast(&program_ast).unwrap();
        core_program
    }

    /// Get the compiled MASM as [`miden_codegen_masm::Program`]
    pub fn ir_masm_program(&mut self) -> Arc<miden_codegen_masm::Program> {
        if self.ir_masm.is_none() {
            let mut compiler = MasmCompiler::new(&self.session);
            let hir = self.hir.take().expect("IR is not compiled");
            let ir_masm = compiler.compile(hir).unwrap();
            let frozen = ir_masm.freeze();
            self.ir_masm = Some(frozen);
        }
        self.ir_masm.clone().unwrap()
    }
}

pub(crate) fn demangle(name: &str) -> String {
    let mut input = name.as_bytes();
    let mut demangled = Vec::new();
    let include_hash = false;
    rustc_demangle::demangle_stream(&mut input, &mut demangled, include_hash).unwrap();
    String::from_utf8(demangled).unwrap()
}

pub(crate) fn wasm_to_wat(wasm_bytes: &[u8]) -> String {
    let mut wasm_printer = wasmprinter::Printer::new();
    // disable printing of the "producers" section because it contains a rustc version
    // to not brake tests when rustc is updated
    wasm_printer.add_custom_section_printer("producers", |_, _, _| Ok(()));
    let wat = wasm_printer.print(wasm_bytes.as_ref()).unwrap();
    wat
}
fn compile_rust_file(rust_source: &str) -> Vec<u8> {
    let rustc_opts = [
        "-C",
        "opt-level=z", // optimize for size
        "--target",
        "wasm32-unknown-unknown",
    ];
    let file_name = hash_string(rust_source);
    let proj_dir = std::env::temp_dir().join(&file_name);
    if proj_dir.exists() {
        fs::remove_dir_all(&proj_dir).unwrap();
        fs::create_dir_all(&proj_dir).unwrap();
    } else {
        fs::create_dir_all(&proj_dir).unwrap();
    }
    let input_file = proj_dir.join(format!("{file_name}.rs"));
    let output_file = proj_dir.join(format!("{file_name}.wasm"));
    fs::write(&input_file, rust_source).unwrap();
    let output = Command::new("rustc")
        .args(&rustc_opts)
        .arg(&input_file)
        .arg("-o")
        .arg(&output_file)
        .output()
        .expect("Failed to execute rustc.");
    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("Rust to Wasm compilation failed!");
    }
    let wasm = fs::read(&output_file).unwrap();
    fs::remove_dir_all(proj_dir).unwrap();
    return wasm;
}

fn default_emitter(verbosity: Verbosity, color: ColorChoice) -> Arc<dyn Emitter> {
    match verbosity {
        Verbosity::Silent => Arc::new(NullEmitter::new(color)),
        _ => Arc::new(DefaultEmitter::new(color)),
    }
}

fn make_diagnostics() -> DiagnosticsHandler {
    let codemap = Arc::new(CodeMap::new());
    let diagnostics = DiagnosticsHandler::new(
        DiagnosticsConfig {
            verbosity: Verbosity::Debug,
            warnings_as_errors: false,
            no_warn: false,
            display: Default::default(),
        },
        codemap,
        default_emitter(Verbosity::Debug, ColorChoice::Auto),
    );
    diagnostics
}

/// Create a default session for testing
pub fn default_session() -> Session {
    let session = Session::new(
        Default::default(),
        InputFile::from_path("test.hir").unwrap(),
        None,
        None,
        None,
        Default::default(),
        None,
    );
    session
}

fn hash_string(inputs: &str) -> String {
    let hash = <sha2::Sha256 as sha2::Digest>::digest(inputs.as_bytes());
    format!("{:x}", hash)
}

fn wasm_to_ir(wasm_bytes: &[u8], session: &Session) -> miden_hir::Module {
    use miden_hir_transform as transforms;
    let mut ir_module = translate_module(
        wasm_bytes,
        &WasmTranslationConfig::default(),
        &session.diagnostics,
    )
    .expect("Failed to translate Wasm to IR module");

    let mut analyses = AnalysisManager::new();
    let mut rewrites = RewriteSet::default();
    rewrites.push(ModuleRewritePassAdapter::new(
        transforms::SplitCriticalEdges,
    ));
    rewrites.push(ModuleRewritePassAdapter::new(transforms::Treeify));
    rewrites.push(ModuleRewritePassAdapter::new(transforms::InlineBlocks));
    rewrites
        .apply(&mut ir_module, &mut analyses, session)
        .expect("Failed to apply rewrites");
    ir_module
}
