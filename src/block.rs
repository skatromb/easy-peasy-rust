use std::io::{Write, stdin, stdout};

use anyhow::Result;
use toml_edit::Item;

use crate::Choices;

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

    const fn docs(self) -> &'static str {
        match self {
            Self::ClippyLint => "https://rust-lang.github.io/rust-clippy/master/index.html",
            Self::RustcLint => "https://doc.rust-lang.org/rustc/lints/listing/index.html",
            Self::ClippySetting => "https://doc.rust-lang.org/clippy/lint_configuration.html",
            Self::Inheritance => {
                "https://doc.rust-lang.org/cargo/reference/workspaces.html#the-lints-table"
            }
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Question {
    Adopt,
    Remove,
}

impl Question {
    const fn prompt(self) -> &'static str {
        match self {
            Self::Adopt => "Adopt? [Y/n]",
            Self::Remove => "Remove? [y/N]",
        }
    }

    const fn default(self) -> bool {
        matches!(self, Self::Adopt)
    }
}

pub(crate) struct Block {
    kind: Kind,
    title: String,
    question: Question,
    lines: Vec<String>,
}

impl Block {
    pub(crate) fn new(kind: Kind, title: &str, question: Question) -> Self {
        Self {
            kind,
            title: title.to_owned(),
            question,
            lines: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, name: &str, yours: Option<&Item>, preset: &Item) {
        let label = self.kind.label(name);
        let wanted = shown(preset);
        self.lines.push(yours.map_or_else(
            || format!("{label}: {wanted}"),
            |mine| format!("{label}: {} → {wanted}", shown(mine)),
        ));
    }

    pub(crate) fn push_name(&mut self, name: &str) {
        self.lines.push(self.kind.label(name));
    }

    pub(crate) fn ask(&self, choices: Choices) -> Result<bool> {
        if self.lines.is_empty() {
            return Ok(false);
        }

        let mut out = stdout().lock();
        writeln!(out, "{}\n{}\n", self.title, self.kind.docs())?;
        for line in &self.lines {
            writeln!(out, "  {line}")?;
        }

        let answer = if choices.diff {
            true
        } else {
            write!(out, "{} ", self.question.prompt())?;
            answer(&mut out, self.question, choices)?
        };
        writeln!(out)?;
        Ok(answer)
    }
}

fn answer(out: &mut impl Write, question: Question, choices: Choices) -> Result<bool> {
    let default = question.default();
    if choices.yes {
        writeln!(out, "{}", if default { "y" } else { "n" })?;
        return Ok(default);
    }

    out.flush()?;
    let line = stdin().lines().next().transpose()?.unwrap_or_default();
    Ok(match line.trim() {
        "" => default,
        typed => typed.eq_ignore_ascii_case("y"),
    })
}

fn shown(setting: &Item) -> String {
    setting.clone().into_value().map_or_else(
        |_| setting.to_string(),
        |mut plain| {
            if let Some(list) = plain.as_array_mut() {
                list.fmt();
            }
            plain.decorated("", "").to_string()
        },
    )
}
