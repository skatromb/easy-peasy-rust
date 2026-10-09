//! `cargo easy-peasy`: installs the easy-peasy-rust lint preset into a Cargo workspace.

mod block;
mod inheritance;
mod merge;
mod toml_file;
mod toolchain;
mod workspace;

use std::io::{IsTerminal as _, Write as _, stdin, stdout};
use std::path::{Path, PathBuf};

use anyhow::{Result, ensure};
use clap::{Args, Parser};
use toml_file::TomlFile;
use toolchain::Toolchain;
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
    /// Default to replacing and removing your own settings. With `--yes`, applies the whole preset.
    #[arg(long)]
    drop_existing: bool,
    /// Print the diff from the preset and fail on any, without writing anything.
    #[arg(long, conflicts_with_all = ["yes", "drop_existing"])]
    diff: bool,
}

fn main() -> Result<()> {
    let Cargo::EasyPeasy(cli_args) = Cargo::parse();
    let choices = cli_args.choices;

    ensure!(
        choices.diff || choices.yes || stdin().is_terminal(),
        "Use `--yes` for a non-interactive run, or `--diff` to only look"
    );
    if !choices.diff && !choices.yes {
        writeln!(
            stdout(),
            "Press Enter for the default answer, or rerun with `--yes` to take all defaults without asking.\n"
        )?;
    }

    let files = merged(&cli_args.path, choices)?;
    if choices.diff {
        return compare(&files);
    }
    save(&files)
}

fn compare(files: &[TomlFile]) -> Result<()> {
    ensure!(
        !files.iter().any(TomlFile::is_changed),
        "Your settings differ from the preset"
    );
    writeln!(stdout(), "Your settings match the preset")?;
    Ok(())
}

fn save(files: &[TomlFile]) -> Result<()> {
    files.iter().try_for_each(TomlFile::save)?;
    writeln!(
        stdout(),
        "Run `cargo clippy --workspace --all-targets` to see what it flags."
    )?;
    Ok(())
}

fn merged(path: &Path, choices: Choices) -> Result<Vec<TomlFile>> {
    let workspace = Workspace::locate(path)?;
    let toolchain = Toolchain::detect(workspace.root())?;
    let mut clippy_toml = workspace.clippy_toml()?;
    let mut skipped = merge::settings(clippy_toml.doc_mut(), choices, &toolchain)?;
    let mut files = vec![clippy_toml];

    if toolchain.reads_lints() {
        let mut cargo_toml = workspace.cargo_toml()?;
        skipped.extend(merge::lints(cargo_toml.doc_mut(), choices, &toolchain)?);
        let mut members = workspace.members()?;
        inheritance::merge(&mut cargo_toml, &mut members, choices)?;
        files.push(cargo_toml);
        files.extend(members);
    }

    skipped.sort();
    toolchain.warn_skipped(&skipped)?;

    Ok(files)
}
