use miden_hir::{
    self as hir,
    pass::{AnalysisManager, ConversionPass, ConversionResult},
    ConversionPassRegistration, PassInfo,
};
use miden_hir_analysis as analysis;
use midenc_session::Session;

use crate::{
    codegen::{FunctionEmitter, OperandStack, Scheduler, TypedValue},
    masm,
};

type ProgramGlobalVariableAnalysis = analysis::GlobalVariableAnalysis<hir::Program>;
type ModuleGlobalVariableAnalysis = analysis::GlobalVariableAnalysis<hir::Module>;

/// Convert an HIR program or module to Miden Assembly
///
/// This pass assumes the following statements are true, and may fail if any are not:
///
/// * The IR has been validated, or is known to be valid
/// * If converting a single module, it must be self-contained
/// * If converting multiple modules, they must be linked into a [Program], in order to
///   ensure that there are no undefined symbols, and that the placement of global variables
///   in linear memory has been fixed.
/// * There are no critical edges in the control flow graph, or the [SplitCriticalEdges]
///   rewrite has been applied.
/// * The control flow graph is a tree, with the exception of loop header blocks. This
///   means that the only blocks with more than one predecessor are loop headers. See
///   the [Treeify] rewrite for more information.
///
/// Any further optimizations or rewrites are considered optional.
#[derive(ConversionPassRegistration)]
pub struct ConvertHirToMasm<T>(core::marker::PhantomData<T>);
impl<T> Default for ConvertHirToMasm<T> {
    fn default() -> Self {
        Self(core::marker::PhantomData)
    }
}
impl<T> PassInfo for ConvertHirToMasm<T> {
    const FLAG: &'static str = "convert-hir-to-masm";
    const SUMMARY: &'static str = "Convert an HIR module or program to Miden Assembly";
    const DESCRIPTION: &'static str = "Convert an HIR module or program to Miden Assembly\n\n\
                                       See the module documentation for ConvertHirToMasm for more details";
}

impl ConversionPass for ConvertHirToMasm<hir::Program> {
    type From = Box<hir::Program>;
    type To = Box<masm::Program>;

    fn convert(
        &mut self,
        mut program: Self::From,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> ConversionResult<Self::To> {
        let mut masm_program = Box::new(masm::Program::from(program.as_ref()));

        // Remove the set of modules to compile from the program
        let modules = program.modules_mut().take();

        // Ensure global variable analysis is computed
        analyses.get_or_compute::<ProgramGlobalVariableAnalysis>(&program, session)?;

        for module in modules.into_iter() {
            // Convert the module
            let mut convert_to_masm = ConvertHirToMasm::<hir::Module>::default();
            let masm_module = convert_to_masm.convert(module, analyses, session)?;

            // If this module makes use of any intrinsics modules, and those modules are not
            // already present, add them to the program.
            for import in masm_module
                .imports
                .iter()
                .filter(|import| import.name.as_str().starts_with("intrinsics::"))
            {
                if masm_program.contains(import.name) {
                    continue;
                }
                match masm::intrinsics::load(import.name.as_str(), &session.codemap) {
                    Some(loaded) => {
                        masm_program.insert(Box::new(loaded));
                    }
                    None => unimplemented!("unrecognized intrinsic module: '{}'", &import.name),
                }
            }

            // Add to the final Miden Assembly program
            masm_program.insert(masm_module);
        }

        Ok(masm_program)
    }
}

impl ConversionPass for ConvertHirToMasm<hir::Module> {
    type From = Box<hir::Module>;
    type To = Box<masm::Module>;

    fn convert(
        &mut self,
        mut module: Self::From,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> ConversionResult<Self::To> {
        use miden_hir::ProgramAnalysisKey;

        let mut masm_module = Box::new(masm::Module::new(module.name));

        // Compute import information for this module
        masm_module.imports = module.imports();

        // If we don't have a program-wide global variable analysis, compute it using the module global table.
        if !analyses.is_available::<ProgramGlobalVariableAnalysis>(&ProgramAnalysisKey) {
            analyses.get_or_compute::<ModuleGlobalVariableAnalysis>(&module, session)?;
        }

        // Removing a function via this cursor will move the cursor to
        // the next function in the module. Once the end of the module
        // is reached, the cursor will point to the null object, and
        // `remove` will return `None`.
        while let Some(function) = module.pop_front() {
            let mut convert_to_masm = ConvertHirToMasm::<&hir::Function>::default();
            let masm_function = convert_to_masm.convert(&function, analyses, session)?;
            masm_module.push_back(Box::new(masm_function));
        }

        Ok(masm_module)
    }
}

impl<'a> ConversionPass for ConvertHirToMasm<&'a hir::Function> {
    type From = &'a hir::Function;
    type To = masm::Function;

    fn convert(
        &mut self,
        f: Self::From,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> ConversionResult<Self::To> {
        use miden_hir::ProgramAnalysisKey;

        let mut f_prime = masm::Function::new(f.id, f.signature.clone());

        // Start at the function entry
        {
            let entry = f.dfg.entry_block();

            let globals = analyses
                .get::<ProgramGlobalVariableAnalysis>(&ProgramAnalysisKey)
                .map(|result| result.layout().clone())
                .unwrap_or_else(|| {
                    let result = analyses.expect::<ModuleGlobalVariableAnalysis>(
                        &f.id.module,
                        "expected global variable analysis to be available",
                    );
                    result.layout().clone()
                });

            let domtree = analyses.get_or_compute::<analysis::DominatorTree>(f, session)?;
            let loops = analyses.get_or_compute::<analysis::LoopAnalysis>(f, session)?;
            let liveness = analyses.get_or_compute::<analysis::LivenessAnalysis>(f, session)?;

            let mut stack = OperandStack::default();
            for arg in f.dfg.block_args(entry).iter().rev().copied() {
                let ty = f.dfg.value_type(arg).clone();
                stack.push(TypedValue { value: arg, ty });
            }

            let scheduler = Scheduler::new(f, &mut f_prime, &domtree, &loops, &liveness);
            let schedule = scheduler.build();

            let emitter =
                FunctionEmitter::new(f, &mut f_prime, &domtree, &loops, &liveness, &globals);
            emitter.emit(schedule, stack);
        }

        Ok(f_prime)
    }
}
