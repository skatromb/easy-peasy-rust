//! `cargo easy-peasy`: installs the easy-peasy-rust lint preset into a Cargo workspace.

mod conflict;
mod merge;
mod toml_file;
mod toolchain;
mod workspace;

use std::io::{IsTerminal as _, Write as _, stdin, stdout};
use std::path::{Path, PathBuf};

use anyhow::{Result, ensure};
use cargo_metadata::semver::Version;
use clap::{Args, Parser};
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
    /// Silently take the preset's value on every conflict.
    #[arg(short = 'y', long)]
    yes: bool,
    /// Drop the lints the preset does not set.
    #[arg(long)]
    drop_existing: bool,
    /// Show the diff from preset, without writing rules.
    #[arg(long, conflicts_with_all = ["yes", "drop_existing"])]
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
        "Use `--yes` for a non-interactive run to accept the preset's value on every conflict"
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

    let mut files = if toolchain::reads_lints(&rust)? {
        lints(&workspace, choices, &rust)?
    } else {
        Vec::new()
    };

    let mut clippy_toml = TomlFile::open(root, workspace.clippy_toml())?;
    merge::settings(clippy_toml.doc_mut(), choices, &rust)?;
    files.push(clippy_toml);
    Ok(files)
}

fn lints(workspace: &Workspace, choices: Choices, rust: &Version) -> Result<Vec<TomlFile>> {
    let root = workspace.root();
    let mut cargo_toml = TomlFile::open(root, "Cargo.toml")?;
    merge::lints(cargo_toml.doc_mut(), choices, rust)?;
    let mut files = vec![cargo_toml];

    for member in workspace.members() {
        let mut member_cargo_toml = TomlFile::open(root, member)?;
        merge::inherit(member_cargo_toml.doc_mut(), member, choices)?;
        files.push(member_cargo_toml);
    }
    Ok(files)
}
