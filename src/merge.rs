use std::iter;

use anyhow::{Context as _, Result, bail};
use toml_edit::{DocumentMut, Item, Key, Table, Value, value};

use crate::Choices;
use crate::block::{Block, Kind, Question};
use crate::supported::Supported;

const LINTS: &str = include_str!("../rules/lints.toml");
const SETTINGS: &str = include_str!("../rules/clippy.toml");

type Entry<'preset> = (&'preset Key, &'preset Item);
type Setting<'preset> = (&'preset Key, Item);
type Missing<'preset> = Vec<Setting<'preset>>;
type Changed<'preset> = Vec<Setting<'preset>>;

pub(crate) fn lints(
    cargo_toml: &mut DocumentMut,
    choices: Choices,
    supported: &Supported,
) -> Result<Vec<String>> {
    let preset: DocumentMut = LINTS.parse()?;
    let path = lints_path(cargo_toml);
    let target = table_at(cargo_toml, path)?;
    let mut skipped = Vec::new();
    for (tool, lints) in preset.iter() {
        let kind = kind(tool)?;
        let mut known = lints
            .as_table()
            .context("preset tool is not a table")?
            .clone();
        skipped.extend(supported.retain(&mut known, tool, kind));
        table(table_at(target, &[tool])?, &known, kind, choices)?;
    }

    if cargo_toml.contains_key("workspace") && cargo_toml.contains_key("package") {
        inherit(cargo_toml, "Cargo.toml", choices)?;
    }
    Ok(skipped)
}

pub(crate) fn inherit(cargo_toml: &mut DocumentMut, name: &str, choices: Choices) -> Result<()> {
    let preset = Item::Table(iter::once(("workspace", value(true))).collect());
    let current = cargo_toml.get("lints");
    if current.is_some_and(|lints| same(lints, &preset)) {
        return Ok(());
    }

    let question = if current.is_some() {
        Question::Replace
    } else {
        Question::Adopt
    };
    let mut block = Block::new(Kind::Inheritance, "Workspace lints");
    block.push(name, current, &preset);
    if block.ask(question, choices)? {
        cargo_toml["lints"] = preset;
    }
    Ok(())
}

pub(crate) fn settings(
    clippy_toml: &mut DocumentMut,
    choices: Choices,
    supported: &Supported,
) -> Result<Vec<String>> {
    let mut preset: DocumentMut = SETTINGS.parse()?;
    let skipped = supported.retain(&mut preset, "clippy.toml", Kind::ClippySetting);

    table(clippy_toml, &preset, Kind::ClippySetting, choices)?;
    Ok(skipped)
}

pub(crate) fn extras(cargo_toml: &mut DocumentMut, choices: Choices) -> Result<()> {
    let preset: DocumentMut = LINTS.parse()?;
    let path = lints_path(cargo_toml);
    let target = table_at(cargo_toml, path)?;
    for (tool, lints) in preset.iter() {
        let known = lints.as_table().context("preset tool is not a table")?;
        remove(table_at(target, &[tool])?, known, tool, choices)?;
    }
    Ok(())
}

fn remove(target: &mut Table, preset: &Table, tool: &str, choices: Choices) -> Result<()> {
    let title = format!("You have {tool} lints that are not in `easy-peasy-rust`");
    let mut block = Block::new(kind(tool)?, &title);
    for (name, _) in target
        .iter()
        .filter(|&(name, _)| !preset.contains_key(name))
    {
        block.push_name(name);
    }

    if block.ask(Question::Remove, choices)? {
        target.retain(|name, _| preset.contains_key(name));
    }
    Ok(())
}

fn lints_path(cargo_toml: &DocumentMut) -> &'static [&'static str] {
    if cargo_toml.contains_key("workspace") {
        &["workspace", "lints"]
    } else {
        &["lints"]
    }
}

fn kind(tool: &str) -> Result<Kind> {
    match tool {
        "clippy" => Ok(Kind::ClippyLint),
        "rust" => Ok(Kind::RustcLint),
        _ => bail!("unknown lint tool `{tool}` in the preset"),
    }
}

fn table(target: &mut Table, preset: &Table, kind: Kind, choices: Choices) -> Result<()> {
    let entries: Vec<_> = entries(preset).collect();
    entries
        .chunk_by(|_, &(key, _)| header(key).is_none())
        .try_for_each(|block| adopt(target, block, kind, choices))
}

fn adopt(target: &mut Table, preset: &[Entry<'_>], kind: Kind, choices: Choices) -> Result<()> {
    let title = preset.first().and_then(|&(key, _)| header(key));
    let mut block = Block::new(kind, title.unwrap_or_default());
    let (missing, changed) = differing(target, preset);

    for (key, setting) in &missing {
        block.push(key.get(), None, setting);
    }
    if block.ask(Question::Adopt, choices)? {
        for (key, setting) in missing {
            drop(target.insert_formatted(key, setting));
        }
    }

    for (key, setting) in &changed {
        block.push(key.get(), target.get(key.get()), setting);
    }
    if block.ask(Question::Replace, choices)? {
        for (key, setting) in changed {
            target[key.get()] = setting;
        }
    }
    Ok(())
}

fn differing<'preset>(
    target: &Table,
    preset: &[Entry<'preset>],
) -> (Missing<'preset>, Changed<'preset>) {
    preset
        .iter()
        .map(|&(key, commented)| (key, uncommented(commented)))
        .filter(|(key, setting)| {
            !target
                .get(key.get())
                .is_some_and(|mine| same(mine, setting))
        })
        .partition(|(key, _)| !target.contains_key(key.get()))
}

fn header(key: &Key) -> Option<&str> {
    key.leaf_decor()
        .prefix()?
        .as_str()?
        .lines()
        .find_map(|line| line.strip_prefix("# "))
}

fn entries(table: &Table) -> impl Iterator<Item = Entry<'_>> {
    table
        .iter()
        .filter_map(|(name, _)| table.get_key_value(name))
}

fn uncommented(setting: &Item) -> Item {
    let mut bare = setting.clone();
    if let Some(plain) = bare.as_value_mut() {
        plain.decor_mut().clear();
    }
    bare
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
