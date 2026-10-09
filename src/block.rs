use std::fmt::Display;
use std::io::{Write, stderr, stdin, stdout};
use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::Result;
use toml_edit::Item;

use crate::Choices;
use crate::toml_file::TomlFile;

static QUIET: AtomicBool = AtomicBool::new(true);

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

    pub(crate) const fn section(self) -> &'static str {
        match self {
            Self::ClippyLint => "clippy",
            Self::RustcLint => "rust",
            Self::ClippySetting => "clippy.toml",
            Self::Inheritance => "lints",
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

struct Change {
    label: String,
    detail: String,
}

pub(crate) struct Block {
    kind: Kind,
    title: String,
    shown: bool,
    changes: Vec<Change>,
}

impl Block {
    pub(crate) fn new(kind: Kind, title: &str) -> Self {
        Self {
            kind,
            title: title.to_owned(),
            shown: false,
            changes: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, name: &str, yours: Option<&Item>, preset: Option<&Item>) {
        let sides: Vec<String> = [yours, preset].into_iter().flatten().map(shown).collect();
        self.changes.push(Change {
            label: self.kind.label(name),
            detail: sides.join(" → "),
        });
    }

    pub(crate) fn ask(&mut self, question: Question, choices: Choices) -> Result<bool> {
        if self.changes.is_empty() {
            return Ok(false);
        }
        QUIET.store(false, Ordering::Relaxed);
        if !choices.interactive && !choices.diff {
            return self.summarize(question, choices);
        }

        let mut out = stdout().lock();
        writeln!(out)?;
        if !self.shown {
            writeln!(out, "  {}\n  {}\n", self.title, self.kind.docs())?;
            self.shown = true;
        }
        for Change { label, detail } in self.changes.drain(..) {
            writeln!(out, "    {label}: {detail}")?;
        }

        Ok(choices.diff || answer(&mut out, question, choices)?)
    }

    fn summarize(&mut self, question: Question, choices: Choices) -> Result<bool> {
        let what = match question {
            Question::Adopt | Question::Remove => self.kind.count(self.changes.len()),
            Question::Replace => {
                let labels: Vec<&str> = self
                    .changes
                    .iter()
                    .map(|change| change.label.as_str())
                    .collect();
                labels.join(", ")
            }
        };
        self.changes.clear();
        let verb = question.verb(choices);
        writeln!(stdout(), "  {verb} {}: {what}", self.title)?;
        Ok(question.default(choices))
    }
}

pub(crate) fn section<T>(
    file: &mut TomlFile,
    merge: impl FnOnce(&mut TomlFile) -> Result<T>,
) -> Result<T> {
    writeln!(stdout(), "{}", file.name())?;
    QUIET.store(true, Ordering::Relaxed);
    let merged = merge(file)?;
    if QUIET.load(Ordering::Relaxed) {
        writeln!(stdout(), "  Matches the preset")?;
    }
    writeln!(stdout())?;
    Ok(merged)
}

fn answer(out: &mut impl Write, question: Question, choices: Choices) -> Result<bool> {
    loop {
        write!(out, "  {} ", question.prompt(choices))?;
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

pub(crate) fn warn(text: impl Display) -> Result<()> {
    writeln!(stderr(), "warning: {text}")?;
    Ok(())
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
