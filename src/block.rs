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

    fn count(self, number: usize) -> String {
        let noun = match self {
            Self::ClippyLint | Self::RustcLint => "lint",
            Self::ClippySetting => "setting",
            Self::Inheritance => "crate",
        };
        let plural = if number == 1 { "" } else { "s" };
        format!("{number} {noun}{plural}")
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

    const fn verb(self, choices: Choices) -> &'static str {
        match self {
            Self::Adopt => "Added",
            Self::Replace if choices.drop_existing => "Replaced",
            Self::Remove if choices.drop_existing => "Removed",
            Self::Replace | Self::Remove => "Kept",
        }
    }
}

type Line = (String, Option<String>);

pub(crate) struct Block {
    kind: Kind,
    title: String,
    shown: bool,
    lines: Vec<Line>,
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
        let wanted = shown(preset);
        let change = yours.map_or_else(
            || wanted.clone(),
            |mine| format!("{} → {wanted}", shown(mine)),
        );
        self.lines.push((self.kind.label(name), Some(change)));
    }

    pub(crate) fn push_name(&mut self, name: &str) {
        self.lines.push((self.kind.label(name), None));
    }

    pub(crate) fn ask(&mut self, question: Question, choices: Choices) -> Result<bool> {
        if self.lines.is_empty() {
            return Ok(false);
        }
        if choices.yes {
            return self.summarize(question, choices);
        }

        let mut out = stdout().lock();
        if !self.shown {
            writeln!(out, "{}\n{}\n", self.title, self.kind.docs())?;
            self.shown = true;
        }
        for (label, change) in self.lines.drain(..) {
            match change {
                Some(detail) => writeln!(out, "  {label}: {detail}")?,
                None => writeln!(out, "  {label}")?,
            }
        }

        let answer = choices.diff || answer(&mut out, question, choices)?;
        writeln!(out)?;
        Ok(answer)
    }

    fn summarize(&mut self, question: Question, choices: Choices) -> Result<bool> {
        let what = match question {
            Question::Adopt => self.kind.count(self.lines.len()),
            Question::Replace | Question::Remove => {
                let labels: Vec<&str> =
                    self.lines.iter().map(|(label, _)| label.as_str()).collect();
                labels.join(", ")
            }
        };
        self.lines.clear();
        let verb = question.verb(choices);
        writeln!(stdout(), "{verb:>12} {}: {what}", self.title)?;
        Ok(question.default(choices))
    }
}

fn answer(out: &mut impl Write, question: Question, choices: Choices) -> Result<bool> {
    loop {
        write!(out, "{} ", question.prompt(choices))?;
        out.flush()?;
        let line = stdin().lines().next().transpose()?.unwrap_or_default();
        match line.trim().to_ascii_lowercase().as_str() {
            "" => return Ok(question.default(choices)),
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => {}
        }
    }
}

pub(crate) fn shown(setting: &Item) -> String {
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
