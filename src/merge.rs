use std::iter;

use anyhow::{Context as _, Result, bail};
use toml_edit::{DocumentMut, Item, Key, Table, Value, value};

use crate::conflict::{Conflict, Kind};

const LINTS: &str = include_str!("../preset/lints.toml");
const SETTINGS: &str = include_str!("../preset/clippy.toml");

pub(crate) fn lints(cargo_toml: &mut DocumentMut, overwrite: bool) -> Result<()> {
    let is_workspace = cargo_toml.contains_key("workspace");
    let path: &[&str] = if is_workspace {
        &["workspace", "lints"]
    } else {
        &["lints"]
    };
    tools(table_at(cargo_toml, path)?, overwrite)?;

    if is_workspace && cargo_toml.contains_key("package") {
        inherit(cargo_toml, "Cargo.toml", overwrite)?;
    }
    Ok(())
}

pub(crate) fn inherit(cargo_toml: &mut DocumentMut, name: &str, overwrite: bool) -> Result<()> {
    let preset = Item::Table(iter::once(("workspace", value(true))).collect());
    match cargo_toml.get_mut("lints") {
        None => drop(cargo_toml.insert("lints", preset)),
        Some(current) if same(current, &preset) => {}
        Some(current) => {
            if Conflict::new(Kind::Inheritance, name, current, &preset).resolve(overwrite)? {
                *current = preset;
            }
        }
    }
    Ok(())
}

pub(crate) fn settings(clippy_toml: &mut DocumentMut, overwrite: bool) -> Result<()> {
    let preset: DocumentMut = SETTINGS.parse()?;
    table(clippy_toml, &preset, Kind::ClippySetting, overwrite)
}

fn tools(target: &mut Table, overwrite: bool) -> Result<()> {
    let preset: DocumentMut = LINTS.parse()?;
    for (tool, lints) in preset.iter() {
        let kind = match tool {
            "clippy" => Kind::ClippyLint,
            "rust" => Kind::RustcLint,
            _ => bail!("unknown lint tool `{tool}` in the preset"),
        };
        let preset_lints = lints.as_table().context("preset tool is not a table")?;
        table(table_at(target, &[tool])?, preset_lints, kind, overwrite)?;
    }
    Ok(())
}

fn table(target: &mut Table, preset: &Table, kind: Kind, overwrite: bool) -> Result<()> {
    for (key, setting) in entries(preset) {
        match target.get_mut(key.get()) {
            None => drop(target.insert_formatted(key, setting.clone())),
            Some(current) if same(current, setting) => {}
            Some(current) => {
                if Conflict::new(kind, key.get(), current, setting).resolve(overwrite)? {
                    *current = setting.clone();
                }
            }
        }
    }
    Ok(())
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
