//! `cargo easy-peasy`: installs the easy-peasy-rust lint preset into a Cargo workspace.

mod merge;

use std::fs;
use std::io::{ErrorKind, IsTerminal as _, Write as _, stdin, stdout};
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, ensure};
use clap::Parser;
use merge::{Conflict, Kind};
use toml_edit::DocumentMut;

const LINTS: &str = include_str!("../preset/lints.toml");
const CLIPPY: &str = include_str!("../preset/clippy.toml");

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum Cargo {
    /// Install the easy-peasy-rust lint preset into a crate or workspace.
    EasyPeasy(Args),
}

#[derive(clap::Args)]
struct Args {
    /// Crate or workspace root.
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Take the preset's value on every conflict.
    #[arg(short = 'y', long)]
    overwrite: bool,
}

fn main() -> Result<()> {
    let Cargo::EasyPeasy(args) = Cargo::parse();
    let manifest = args.path.join("Cargo.toml");

    ensure!(
        manifest.is_file(),
        "no Cargo.toml in {}",
        args.path.display()
    );
    ensure!(
        args.overwrite || stdin().is_terminal(),
        "stdin is not a terminal, so conflicts can't be asked about; \
         pass --overwrite to take the preset's value on every conflict"
    );

    install(&args, &manifest)?;

    writeln!(
        stdout(),
        "Run `cargo clippy --all-targets` to see what it flags."
    )?;

    Ok(())
}

fn install(args: &Args, manifest: &Path) -> Result<()> {
    let lints: DocumentMut = LINTS.parse()?;
    let clippy: DocumentMut = CLIPPY.parse()?;
    let mut decide = |conflict: &Conflict| resolve(conflict, args.overwrite);

    let mut cargo_toml = load(manifest)?;
    merge::lints(&mut cargo_toml, &lints, &mut decide)?;
    save(manifest, &cargo_toml)?;

    let config = args.path.join("clippy.toml");
    let mut clippy_toml = load(&config)?;
    merge::table(&mut clippy_toml, &clippy, Kind::ClippySetting, &mut decide)?;
    save(&config, &clippy_toml)
}

fn resolve(conflict: &Conflict, overwrite: bool) -> Result<bool> {
    let mut out = stdout().lock();
    writeln!(out, "{conflict}")?;

    if overwrite {
        writeln!(out, "Replaced yours")?;
        return Ok(true);
    }

    write!(out, "Take the preset's value? [y/N] ")?;
    out.flush()?;
    let answer = stdin().lines().next().transpose()?.unwrap_or_default();

    Ok(answer.trim().eq_ignore_ascii_case("y"))
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
