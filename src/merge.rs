use std::fmt::{self, Display, Formatter};

use anyhow::{Context as _, Result, bail};
use toml_edit::{DocumentMut, Item, Key, RawString, Table, Value};

const CLIPPY_GROUPS: [&str; 3] = ["nursery", "pedantic", "restriction"];

#[derive(Clone, Copy)]
pub(crate) enum Kind {
    ClippyLint,
    RustcLint,
    ClippySetting,
}

pub(crate) struct Conflict {
    kind: Kind,
    key: String,
    yours: String,
    preset: String,
}

impl Conflict {
    fn docs(&self) -> String {
        let key = &self.key;
        match self.kind {
            Kind::ClippyLint if CLIPPY_GROUPS.contains(&key.as_str()) => {
                format!("https://doc.rust-lang.org/clippy/lints.html#{key}")
            }
            Kind::ClippyLint => {
                format!("https://rust-lang.github.io/rust-clippy/master/index.html#{key}")
            }
            Kind::RustcLint => format!(
                "https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#{}",
                key.replace('_', "-")
            ),
            Kind::ClippySetting => {
                format!("https://doc.rust-lang.org/clippy/lint_configuration.html#{key}")
            }
        }
    }
}

impl Display for Conflict {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let prefix = if matches!(self.kind, Kind::ClippyLint) {
            "clippy::"
        } else {
            ""
        };
        let Self {
            key, yours, preset, ..
        } = self;
        write!(
            f,
            "{prefix}{key}: yours {yours}, preset {preset}\n  {}",
            self.docs()
        )
    }
}

pub(crate) fn lints(
    manifest: &mut DocumentMut,
    preset: &DocumentMut,
    resolve: &mut impl FnMut(&Conflict) -> Result<bool>,
) -> Result<()> {
    let root: &[&str] = if manifest.contains_key("workspace") {
        &["workspace", "lints"]
    } else {
        &["lints"]
    };
    for (tool, sections) in preset.iter() {
        let kind = match tool {
            "clippy" => Kind::ClippyLint,
            "rust" => Kind::RustcLint,
            _ => bail!("unknown lint tool `{tool}` in the preset"),
        };
        let target = table_at(manifest.as_table_mut(), &[root, &[tool]].concat())?;
        let flat = flatten(sections.as_table().context("preset tool is not a table")?);
        table(target, &flat, kind, resolve)?;
    }
    Ok(())
}

pub(crate) fn table(
    target: &mut Table,
    preset: &Table,
    kind: Kind,
    resolve: &mut impl FnMut(&Conflict) -> Result<bool>,
) -> Result<()> {
    for (key, setting) in entries(preset) {
        match target.get_mut(key.get()) {
            None => drop(target.insert_formatted(key, setting.clone())),
            Some(current) if same(current, setting) => {}
            Some(current) => {
                let conflict = Conflict {
                    kind,
                    key: key.get().to_owned(),
                    yours: undecorated(current),
                    preset: undecorated(setting),
                };
                if resolve(&conflict)? {
                    *current = setting.clone();
                }
            }
        }
    }
    Ok(())
}

fn flatten(sections: &Table) -> Table {
    let mut flat = Table::new();
    for section in sections.iter().filter_map(|(_, entry)| entry.as_table()) {
        let gap = if flat.is_empty() { "" } else { "\n" };
        let mut header = section
            .decor()
            .prefix()
            .and_then(RawString::as_str)
            .map(|comment| format!("{gap}{}", comment.trim_start()));
        for (key, lint) in entries(section) {
            let mut leaf = key.clone();
            if let Some(comment) = header.take() {
                leaf.leaf_decor_mut().set_prefix(comment);
            }
            drop(flat.insert_formatted(&leaf, lint.clone()));
        }
    }
    flat
}

fn entries(table: &Table) -> impl Iterator<Item = (&Key, &Item)> {
    table
        .iter()
        .filter_map(|(name, _)| table.get_key_value(name))
}

fn table_at<'doc>(root: &'doc mut Table, path: &[&str]) -> Result<&'doc mut Table> {
    path.iter().try_fold(root, |parent, name| {
        parent
            .entry(name)
            .or_insert_with(implicit_table)
            .as_table_mut()
            .with_context(|| format!("`{name}` in Cargo.toml is not a table"))
    })
}

fn implicit_table() -> Item {
    let mut table = Table::new();
    table.set_implicit(true);
    Item::Table(table)
}

fn undecorated(setting: &Item) -> String {
    setting.as_value().map_or_else(
        || setting.to_string(),
        |plain| plain.clone().decorated("", "").to_string(),
    )
}

fn same(current: &Item, setting: &Item) -> bool {
    current
        .as_value()
        .zip(setting.as_value())
        .is_some_and(|(mine, theirs)| equal(mine, theirs))
}

fn equal(mine: &Value, theirs: &Value) -> bool {
    match (mine, theirs) {
        (Value::String(left), Value::String(right)) => left.value() == right.value(),
        (Value::Integer(left), Value::Integer(right)) => left.value() == right.value(),
        (Value::Boolean(left), Value::Boolean(right)) => left.value() == right.value(),
        (Value::Array(left), Value::Array(right)) => {
            left.len() == right.len()
                && left.iter().zip(right).all(|(one, other)| equal(one, other))
        }
        (Value::InlineTable(left), Value::InlineTable(right)) => {
            left.len() == right.len()
                && left
                    .iter()
                    .all(|(name, one)| right.get(name).is_some_and(|other| equal(one, other)))
        }
        _ => false,
    }
}
