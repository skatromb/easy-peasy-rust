//! `cargo easy-peasy`: installs the easy-peasy-rust lint preset into a Cargo workspace.

mod block;
mod merge;
mod supported;
mod toml_file;
mod toolchain;
mod workspace;

use std::io::{IsTerminal as _, Write as _, stdin, stdout};
use std::path::{Path, PathBuf};

use anyhow::{Result, ensure};
use clap::{Args, Parser};
use supported::Supported;
use toml_file::TomlFile;
use workspace::Workspace;

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum Cargo {
    /// Install the easy-peasy-rust lint preset into a crate or workspace.
    #[command(version)]
    EasyPeasy(CliArgs),
}

#[derive(Args)]
struct CliArgs {
    /// Any directory inside the crate or workspace.
    #[arg(default_value = ".")]
    path: PathBuf,
    #[command(flatten)]
    choices: Choices,
}

#[derive(Args, Clone, Copy)]
struct Choices {
    /// Add what the preset sets and you lack, without asking. Keeps your own settings.
    #[arg(short = 'y', long)]
    yes: bool,
    /// Print the diff from the preset, without writing anything.
    #[arg(long, conflicts_with = "yes")]
    diff: bool,
}

fn main() -> Result<()> {
    let Cargo::EasyPeasy(cli_args) = Cargo::parse();
    let choices = cli_args.choices;

    if choices.diff {
        return install(&cli_args.path, choices);
    }

    ensure!(
        stdin().is_terminal() || choices.yes,
        "Use `--yes` for a non-interactive run"
    );

    install(&cli_args.path, choices)?;
    writeln!(
        stdout(),
        "Run `cargo clippy --workspace --all-targets` to see what it flags."
    )?;

    Ok(())
}

fn install(path: &Path, choices: Choices) -> Result<()> {
    let files = merged(path, choices)?;

    if !choices.diff {
        return files.iter().try_for_each(TomlFile::save);
    }

    if !files.iter().any(TomlFile::is_changed) {
        writeln!(stdout(), "Your settings match the preset")?;
    }

    Ok(())
}

fn merged(path: &Path, choices: Choices) -> Result<Vec<TomlFile>> {
    let workspace = Workspace::locate(path)?;
    let root = workspace.root();
    let rust = toolchain::rust_release(root)?;
    let supported = Supported::new(&rust)?;
    let mut clippy_toml = TomlFile::open(root, workspace.clippy_toml())?;
    let mut skipped = merge::settings(clippy_toml.doc_mut(), choices, &supported)?;
    let mut files = vec![clippy_toml];

    if toolchain::reads_lints(&rust)? {
        let mut cargo_toml = TomlFile::open(root, "Cargo.toml")?;
        skipped.extend(merge::lints(cargo_toml.doc_mut(), choices, &supported)?);
        let members = members(&workspace, choices)?;
        merge::extras(cargo_toml.doc_mut(), choices)?;
        files.push(cargo_toml);
        files.extend(members);
    }

    skipped.sort();
    toolchain::warn_skipped(&rust, &skipped)?;

    Ok(files)
}

fn members(workspace: &Workspace, choices: Choices) -> Result<Vec<TomlFile>> {
    let root = workspace.root();
    workspace
        .members()
        .iter()
        .map(|member| {
            let mut cargo_toml = TomlFile::open(root, member)?;
            merge::inherit(cargo_toml.doc_mut(), member, choices)?;
            Ok(cargo_toml)
        })
        .collect()
}
