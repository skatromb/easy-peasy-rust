//! `cargo easy-peasy`: installs the easy-peasy-rust lint preset into a Cargo workspace.

mod conflict;
mod merge;
mod toml_file;
mod toolchain;
mod workspace;

use std::io::{IsTerminal as _, Write as _, stdin, stdout};
use std::path::{Path, PathBuf};

use anyhow::{Result, ensure};
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
    /// Answer yes to every question: take the preset's value on every conflict.
    #[arg(short = 'y', long)]
    yes: bool,
    /// Drop the lints the preset does not set, asking about each unless `--yes`.
    #[arg(long)]
    drop_existing: bool,
}

fn main() -> Result<()> {
    let Cargo::EasyPeasy(cli_args) = Cargo::parse();

    ensure!(
        stdin().is_terminal() || cli_args.choices.yes,
        "Use `--yes` for a non-interactive run to accept the preset's value on every conflict"
    );

    install(&cli_args.path, cli_args.choices)?;

    writeln!(
        stdout(),
        "Run `cargo clippy --workspace --all-targets` to see what it flags."
    )?;

    Ok(())
}

fn install(path: &Path, choices: Choices) -> Result<()> {
    let workspace = Workspace::locate(path)?;
    let root = workspace.root();
    toolchain::warn_if_older(root)?;

    let mut cargo_toml = TomlFile::open(root, "Cargo.toml")?;
    merge::lints(cargo_toml.doc_mut(), choices)?;
    cargo_toml.save()?;

    for member in workspace.members() {
        let mut member_cargo_toml = TomlFile::open(root, member)?;
        merge::inherit(member_cargo_toml.doc_mut(), member, choices)?;
        member_cargo_toml.save()?;
    }

    let mut clippy_toml = TomlFile::open(root, workspace.clippy_toml())?;
    merge::settings(clippy_toml.doc_mut(), choices)?;

    clippy_toml.save()
}
