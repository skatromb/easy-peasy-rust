//! `cargo easy-peasy`: installs the easy-peasy-rust lint preset into a Cargo workspace.

mod merge;

use std::fs;
use std::io::{ErrorKind, IsTerminal as _, Write as _, stdin, stdout};
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, ensure};
use clap::{Args, Parser};
use merge::Kind;
use toml_edit::DocumentMut;

const LINTS: &str = include_str!("../preset/lints.toml");
const CLIPPY: &str = include_str!("../preset/clippy.toml");

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum Cargo {
    /// Install the easy-peasy-rust lint preset into a crate or workspace.
    EasyPeasy(CliArgs),
}

#[derive(Args)]
struct CliArgs {
    /// Crate or workspace root.
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
        "Run `cargo clippy --all-targets` to see what it flags."
    )?;

    Ok(())
}

fn install(path: &Path, overwrite: bool) -> Result<()> {
    let cargo_toml_path = path.join("Cargo.toml");
    let clippy_toml_path = path.join("clippy.toml");

    ensure!(
        cargo_toml_path.is_file(),
        "no Cargo.toml in {}",
        path.display()
    );

    let lints: DocumentMut = LINTS.parse()?;
    let clippy: DocumentMut = CLIPPY.parse()?;

    let mut cargo_toml = load(&cargo_toml_path)?;
    merge::lints(&mut cargo_toml, &lints, overwrite)?;
    save(&cargo_toml_path, &cargo_toml)?;

    let mut clippy_toml = load(&clippy_toml_path)?;
    merge::table(&mut clippy_toml, &clippy, Kind::ClippySetting, overwrite)?;
    save(&clippy_toml_path, &clippy_toml)
}

fn load(path: &Path) -> Result<DocumentMut> {
    Ok(read(path)?.unwrap_or_default().parse()?)
}

fn save(path: &Path, doc: &DocumentMut) -> Result<()> {
    let after = doc.to_string();

    let status = match read(path)? {
        None => "Created",
        Some(before) if before == after => "Unchanged",
        Some(_) => "Updated",
    };

    if status != "Unchanged" {
        fs::write(path, after).with_context(|| format!("writing {}", path.display()))?;
    }

    writeln!(stdout(), "{status} {}", path.display())?;
    Ok(())
}

fn read(path: &Path) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("reading {}", path.display())),
    }
}
