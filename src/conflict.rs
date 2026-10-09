use std::fmt::{self, Display, Formatter};
use std::io::{Write as _, stderr, stdin, stdout};

use anyhow::Result;
use toml_edit::Item;

use crate::Choices;

const CLIPPY_GROUPS: [&str; 9] = [
    "cargo",
    "complexity",
    "correctness",
    "nursery",
    "pedantic",
    "perf",
    "restriction",
    "style",
    "suspicious",
];

#[derive(Clone, Copy)]
pub(crate) enum Kind {
    ClippyLint,
    RustcLint,
    ClippySetting,
    Inheritance,
}

impl Kind {
    pub(crate) fn label(self, name: &str) -> String {
        match self {
            Self::ClippyLint => format!("clippy::{name}"),
            Self::RustcLint | Self::ClippySetting => name.to_owned(),
            Self::Inheritance => format!("[lints] in {name}"),
        }
    }
}

pub(crate) struct Conflict {
    kind: Kind,
    name: String,
    yours: String,
    preset: String,
}

impl Conflict {
    pub(crate) fn new(kind: Kind, name: &str, yours: Option<&Item>, preset: Option<&Item>) -> Self {
        Self {
            kind,
            name: name.to_owned(),
            yours: shown(yours),
            preset: shown(preset),
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn show(&self) -> Result<()> {
        writeln!(stdout(), "{self}")?;
        Ok(())
    }

    pub(crate) fn resolve(&self, choices: Choices) -> Result<bool> {
        let mut out = stdout().lock();
        writeln!(out, "{self}")?;

        if choices.diff {
            return Ok(true);
        }

        if choices.yes {
            writeln!(out, "Replaced yours")?;
            return Ok(true);
        }

        write!(out, "Take the preset's value? [Y/n] ")?;
        out.flush()?;
        let answer = stdin().lines().next().transpose()?.unwrap_or_default();

        Ok(!answer.trim().eq_ignore_ascii_case("n"))
    }

    fn docs(&self) -> String {
        let name = &self.name;
        match self.kind {
            Kind::ClippyLint if CLIPPY_GROUPS.contains(&name.as_str()) => {
                format!("https://doc.rust-lang.org/clippy/lints.html#{name}")
            }
            Kind::ClippyLint => {
                format!("https://rust-lang.github.io/rust-clippy/master/index.html#{name}")
            }
            Kind::RustcLint => format!(
                "https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#{}",
                name.replace('_', "-")
            ),
            Kind::ClippySetting => {
                format!("https://doc.rust-lang.org/clippy/lint_configuration.html#{name}")
            }
            Kind::Inheritance => {
                "https://doc.rust-lang.org/cargo/reference/workspaces.html#the-lints-table"
                    .to_owned()
            }
        }
    }
}

impl Display for Conflict {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self {
            kind,
            name,
            yours,
            preset,
        } = self;
        write!(
            f,
            "{}: yours {yours}, preset {preset}\n  {}",
            kind.label(name),
            self.docs()
        )
    }
}

pub(crate) fn warn_kept(kept: &[Conflict]) -> Result<()> {
    if kept.is_empty() {
        return Ok(());
    }

    let mut out = stderr().lock();
    writeln!(
        out,
        "warning: kept your lints that the preset does not set:"
    )?;
    for conflict in kept {
        writeln!(out, "{conflict}")?;
    }
    writeln!(
        out,
        "To drop them, run `cargo easy-peasy --drop-existing [--yes]`"
    )?;
    Ok(())
}

fn shown(setting: Option<&Item>) -> String {
    setting.map_or_else(|| "not set".to_owned(), undecorated)
}

fn undecorated(setting: &Item) -> String {
    setting.clone().into_value().map_or_else(
        |_| setting.to_string(),
        |plain| plain.decorated("", "").to_string(),
    )
}
