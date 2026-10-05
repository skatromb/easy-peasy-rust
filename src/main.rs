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
    EasyPeasy(CliArgs),
}

#[derive(Args)]
struct CliArgs {
    /// Any directory inside the crate or workspace.
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Take the preset's value on every conflict.
    #[arg(short = 'y', long)]
    overwrite: bool,
}

fn main() -> Result<()> {
    let Cargo::EasyPeasy(cli_args) = Cargo::parse();

    ensure!(
        cli_args.overwrite || stdin().is_terminal(),
        "Runs as non-interactive — use `--overwrite` to overwrite all lint settings"
    );

    install(&cli_args.path, cli_args.overwrite)?;

    writeln!(
        stdout(),
        "Run `cargo clippy --workspace --all-targets` to see what it flags."
    )?;

    Ok(())
}

fn install(path: &Path, overwrite: bool) -> Result<()> {
    let workspace = Workspace::locate(path)?;
    let root = workspace.root();
    toolchain::warn_if_older(root)?;

    let mut cargo_toml = TomlFile::open(root, "Cargo.toml")?;
    merge::lints(cargo_toml.doc_mut(), overwrite)?;
    cargo_toml.save()?;

    for member in workspace.members() {
        let mut member_toml = TomlFile::open(root, member)?;
        merge::inherit(member_toml.doc_mut(), member, overwrite)?;
        member_toml.save()?;
    }

    let mut clippy_toml = TomlFile::open(root, workspace.clippy_toml())?;
    merge::settings(clippy_toml.doc_mut(), overwrite)?;
    clippy_toml.save()
}
