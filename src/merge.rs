use std::iter;

use anyhow::{Context as _, Result, bail};
use toml_edit::{DocumentMut, Item, Key, Table, Value, value};

use crate::Choices;
use crate::conflict::{self, Conflict, Kind};

const LINTS: &str = include_str!("../rules/lints.toml");
const SETTINGS: &str = include_str!("../rules/clippy.toml");

pub(crate) fn lints(cargo_toml: &mut DocumentMut, choices: Choices) -> Result<()> {
    let is_workspace = cargo_toml.contains_key("workspace");
    let path: &[&str] = if is_workspace {
        &["workspace", "lints"]
    } else {
        &["lints"]
    };
    let kept = tools(table_at(cargo_toml, path)?, choices)?;
    conflict::warn_kept(&kept)?;

    if is_workspace && cargo_toml.contains_key("package") {
        inherit(cargo_toml, "Cargo.toml", choices)?;
    }
    Ok(())
}

pub(crate) fn inherit(cargo_toml: &mut DocumentMut, name: &str, choices: Choices) -> Result<()> {
    let preset = Item::Table(iter::once(("workspace", value(true))).collect());
    match cargo_toml.get_mut("lints") {
        None => drop(cargo_toml.insert("lints", preset)),
        Some(current) if same(current, &preset) => {}
        Some(current) => {
            if Conflict::new(Kind::Inheritance, name, current, &preset).resolve(choices)? {
                *current = preset;
            }
        }
    }
    Ok(())
}

pub(crate) fn settings(clippy_toml: &mut DocumentMut, choices: Choices) -> Result<()> {
    let preset: DocumentMut = SETTINGS.parse()?;
    table(clippy_toml, &preset, Kind::ClippySetting, choices)
}

fn tools(target: &mut Table, choices: Choices) -> Result<Vec<Conflict>> {
    let preset: DocumentMut = LINTS.parse()?;
    let mut kept = Vec::new();
    for (tool, lints) in preset.iter() {
        let kind = match tool {
            "clippy" => Kind::ClippyLint,
            "rust" => Kind::RustcLint,
            _ => bail!("unknown lint tool `{tool}` in the preset"),
        };
        let preset_lints = lints.as_table().context("preset tool is not a table")?;
        let tool_lints = table_at(target, &[tool])?;
        table(tool_lints, preset_lints, kind, choices)?;
        kept.extend(existing(tool_lints, preset_lints, kind, choices)?);
    }
    Ok(kept)
}

fn table(target: &mut Table, preset: &Table, kind: Kind, choices: Choices) -> Result<()> {
    for (key, setting) in entries(preset) {
        match target.get_mut(key.get()) {
            None => drop(target.insert_formatted(key, setting.clone())),
            Some(current) if same(current, setting) => {}
            Some(current) => {
                if Conflict::new(kind, key.get(), current, setting).resolve(choices)? {
                    *current = setting.clone();
                }
            }
        }
    }
    Ok(())
}

fn existing(
    target: &mut Table,
    preset: &Table,
    kind: Kind,
    choices: Choices,
) -> Result<Vec<Conflict>> {
    let unset: Vec<Conflict> = target
        .iter()
        .filter(|&(name, _)| !preset.contains_key(name))
        .map(|(name, current)| Conflict::unset(kind, name, current))
        .collect();
    if !choices.drop_existing {
        return Ok(unset);
    }

    for conflict in unset {
        if conflict.resolve(choices)? {
            drop(target.remove(conflict.name()));
        }
    }
    Ok(Vec::new())
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

fn same(current: &Item, setting: &Item) -> bool {
    match (current.clone().into_value(), setting.clone().into_value()) {
        (Ok(mine), Ok(theirs)) => equal(&mine, &theirs),
        _ => false,
    }
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
