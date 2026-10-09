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
            Self::RustcLint | Self::ClippySetting | Self::Inheritance => name.to_owned(),
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
    Replace,
    Remove,
}

impl Question {
    fn prompt(self, choices: Choices) -> String {
        let question = match self {
            Self::Adopt => "Adopt?",
            Self::Replace => "Replace yours?",
            Self::Remove => "Remove?",
        };
        let hint = if self.default(choices) {
            "[Y/n]"
        } else {
            "[y/N]"
        };
        format!("{question} {hint}")
    }

    const fn default(self, choices: Choices) -> bool {
        matches!(self, Self::Adopt) || choices.drop_existing
    }
}

pub(crate) struct Block {
    kind: Kind,
    title: String,
    shown: bool,
    lines: Vec<String>,
}

impl Block {
    pub(crate) fn new(kind: Kind, title: &str) -> Self {
        Self {
            kind,
            title: title.to_owned(),
            shown: false,
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

    pub(crate) fn ask(&mut self, question: Question, choices: Choices) -> Result<bool> {
        if self.lines.is_empty() {
            return Ok(false);
        }

        let mut out = stdout().lock();
        if !self.shown {
            writeln!(out, "{}\n{}\n", self.title, self.kind.docs())?;
            self.shown = true;
        }
        for line in self.lines.drain(..) {
            writeln!(out, "  {line}")?;
        }

        let answer = if choices.diff {
            true
        } else {
            write!(out, "{} ", question.prompt(choices))?;
            answer(&mut out, question, choices)?
        };
        writeln!(out)?;
        Ok(answer)
    }
}

fn answer(out: &mut impl Write, question: Question, choices: Choices) -> Result<bool> {
    let default = question.default(choices);
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
