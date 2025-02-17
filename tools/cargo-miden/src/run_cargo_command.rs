use anyhow::bail;
use cargo_metadata::Metadata;
use std::path::PathBuf;
use std::process::Command;

use crate::build::build_masm;
use crate::config::CargoArguments;
use crate::target::{install_wasm32_wasi, WASM32_WASI_TARGET};

fn is_wasm_target(target: &str) -> bool {
    target == WASM32_WASI_TARGET
}

/// Runs the cargo command as specified in the configuration.
///
/// Returns any relevant output artifacts.
pub fn run_cargo_command(
    metadata: &Metadata,
    subcommand: Option<&str>,
    cargo_args: &CargoArguments,
    spawn_args: &[String],
) -> anyhow::Result<Vec<PathBuf>> {
    let cargo = std::env::var("CARGO")
        .map(PathBuf::from)
        .ok()
        .unwrap_or_else(|| PathBuf::from("cargo"));

    let mut args = spawn_args.iter().peekable();
    if let Some(arg) = args.peek() {
        if *arg == "miden" {
            args.next().unwrap();
        }
    }

    // Spawn the actual cargo command
    log::debug!(
        "spawning cargo `{cargo}` with arguments `{args:?}`",
        cargo = cargo.display(),
        args = args.clone().collect::<Vec<_>>(),
    );

    let mut cmd = Command::new(&cargo);
    cmd.args(args);

    let is_build = matches!(subcommand, Some("b") | Some("build"));

    // Handle the target for build commands
    if is_build {
        install_wasm32_wasi()?;

        // Add an implicit wasm32-wasi target if there isn't a wasm target present
        if !cargo_args.targets.iter().any(|t| is_wasm_target(t)) {
            cmd.arg("--target").arg(WASM32_WASI_TARGET);
        }
    }

    match cmd.status() {
        Ok(status) => {
            if !status.success() {
                bail!("cargo failed with exit code {}", status.code().unwrap_or(1));
            }
        }
        Err(e) => {
            bail!("failed to spawn `{cargo}`: {e}", cargo = cargo.display());
        }
    }
    let mut outputs = Vec::new();
    if is_build {
        log::debug!("searching for WebAssembly modules to compile to MASM");
        let targets = cargo_args
            .targets
            .iter()
            .map(String::as_str)
            .filter(|t| is_wasm_target(t))
            .chain(cargo_args.targets.is_empty().then_some(WASM32_WASI_TARGET));

        for target in targets {
            let out_dir = metadata
                .target_directory
                .join(target)
                .join(if cargo_args.release {
                    "release"
                } else {
                    "debug"
                });

            let miden_out_dir =
                metadata
                    .target_directory
                    .join("miden")
                    .join(if cargo_args.release {
                        "release"
                    } else {
                        "debug"
                    });
            if !miden_out_dir.exists() {
                std::fs::create_dir_all(&miden_out_dir)?;
            }

            for package in &metadata.packages {
                let is_bin = package.targets.iter().any(|t| t.is_bin());

                // First try for <name>.wasm
                let path = out_dir.join(&package.name).with_extension("wasm");
                if path.exists() {
                    let output =
                        build_masm(path.as_std_path(), miden_out_dir.as_std_path(), is_bin)?;
                    outputs.push(output);
                } else {
                    let path = out_dir
                        .join(package.name.replace('-', "_"))
                        .with_extension("wasm");
                    if path.exists() {
                        let output =
                            build_masm(path.as_std_path(), miden_out_dir.as_std_path(), is_bin)?;
                        outputs.push(output);
                    } else {
                        log::debug!("no output found for package `{name}`", name = package.name);
                        bail!("Cargo build failed, no Wasm artifact found");
                    }
                }
            }
        }
    }

    Ok(outputs)
}
