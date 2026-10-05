use std::fmt::{self, Display, Formatter};
use std::io::{Write as _, stdin, stdout};

use anyhow::Result;
use toml_edit::Item;

const CLIPPY_GROUPS: [&str; 3] = ["nursery", "pedantic", "restriction"];

#[derive(Clone, Copy)]
pub(crate) enum Kind {
    ClippyLint,
    RustcLint,
    ClippySetting,
    Inheritance,
}

pub(crate) struct Conflict {
    kind: Kind,
    name: String,
    yours: String,
    preset: String,
}

impl Conflict {
    pub(crate) fn new(kind: Kind, name: &str, yours: &Item, preset: &Item) -> Self {
        Self {
            kind,
            name: name.to_owned(),
            yours: undecorated(yours),
            preset: undecorated(preset),
        }
    }

    pub(crate) fn resolve(&self, overwrite: bool) -> Result<bool> {
        let mut out = stdout().lock();
        writeln!(out, "{self}")?;

        if overwrite {
            writeln!(out, "Replaced yours")?;
            return Ok(true);
        }

        write!(out, "Take the preset's value? [y/N] ")?;
        out.flush()?;
        let answer = stdin().lines().next().transpose()?.unwrap_or_default();

        Ok(answer.trim().eq_ignore_ascii_case("y"))
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
        match kind {
            Kind::ClippyLint => write!(f, "clippy::{name}"),
            Kind::RustcLint | Kind::ClippySetting => write!(f, "{name}"),
            Kind::Inheritance => write!(f, "[lints] in {name}"),
        }?;
        write!(f, ": yours {yours}, preset {preset}\n  {}", self.docs())
    }
}

fn undecorated(setting: &Item) -> String {
    setting.clone().into_value().map_or_else(
        |_| setting.to_string(),
        |plain| plain.decorated("", "").to_string(),
    )
}
