use std::mem;

use anyhow::{Context as _, Result};
use toml_edit::{DocumentMut, Item, Key, Table, Value};

use crate::Choices;
use crate::block::{Block, Kind, Question};
use crate::toolchain::Toolchain;

const LINTS: &str = include_str!("../rules/lints.toml");
const SETTINGS: &str = include_str!("../rules/clippy.toml");
const TOOLS: [Kind; 2] = [Kind::RustcLint, Kind::ClippyLint];

type Rule<'preset> = (&'preset Key, &'preset Item);
type Setting<'preset> = (&'preset Key, Item);
pub(crate) type Split<T> = (Vec<T>, Vec<T>);

pub(crate) fn settings(
    clippy_toml: &mut DocumentMut,
    choices: Choices,
    toolchain: &Toolchain,
) -> Result<Vec<String>> {
    let mut preset: DocumentMut = SETTINGS.parse()?;
    let skipped = toolchain.retain(&mut preset, Kind::ClippySetting);

    table(clippy_toml, &preset, Kind::ClippySetting, choices)?;
    Ok(skipped)
}

pub(crate) fn lints(
    cargo_toml: &mut DocumentMut,
    choices: Choices,
    toolchain: &Toolchain,
) -> Result<Vec<String>> {
    let preset: DocumentMut = LINTS.parse()?;
    let path = lints_path(cargo_toml);
    let target = table_at(cargo_toml, path)?;
    let mut skipped = Vec::new();
    for kind in TOOLS {
        let tool = kind.section();
        let all = preset
            .get(tool)
            .and_then(Item::as_table)
            .with_context(|| format!("`[{tool}]` missing from lints.toml"))?;
        let mut known = all.clone();
        skipped.extend(toolchain.retain(&mut known, kind));
        let yours = table_at(target, &[tool])?;
        table(yours, &known, kind, choices)?;
        remove(yours, all, kind, choices)?;
    }
    Ok(skipped)
}

fn remove(target: &mut Table, preset: &Table, kind: Kind, choices: Choices) -> Result<()> {
    let title = format!("Your {} lints not in `easy-peasy-rust`", kind.section());
    let mut block = Block::new(kind, &title);
    for (name, _) in target
        .iter()
        .filter(|&(name, _)| !preset.contains_key(name))
    {
        block.push(name);
    }

    if block.ask(Question::Remove, choices)? {
        target.retain(|name, _| preset.contains_key(name));
    }
    Ok(())
}

fn table(target: &mut Table, preset: &Table, kind: Kind, choices: Choices) -> Result<()> {
    let wanted: Vec<_> = rules(preset).collect();
    let existing = target.iter().map(|(name, _)| name.to_owned()).collect();
    wanted
        .chunk_by(|_, &(key, _)| header(key).is_none())
        .try_for_each(|block| adopt(target, block, kind, choices))?;
    arrange(target, &wanted, existing);
    Ok(())
}

fn adopt(target: &mut Table, preset: &[Rule<'_>], kind: Kind, choices: Choices) -> Result<()> {
    let title = preset.first().and_then(|&(key, _)| header(key));
    let mut block = Block::new(kind, title.unwrap_or_default());
    let (missing, changed) = differing(target, preset);

    for (key, setting) in &missing {
        block.push_setting(key.get(), None, setting);
    }
    if block.ask(Question::Adopt, choices)? {
        for (key, setting) in missing {
            drop(target.insert_formatted(key, setting));
        }
    }

    for (key, setting) in &changed {
        block.push_setting(key.get(), target.get(key.get()), setting);
    }
    if block.ask(Question::Replace, choices)? {
        for (key, setting) in changed {
            target[key.get()] = setting;
        }
    }
    Ok(())
}

fn differing<'preset>(target: &Table, preset: &[Rule<'preset>]) -> Split<Setting<'preset>> {
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

fn arrange(target: &mut Table, preset: &[Rule<'_>], mut order: Vec<String>) {
    for (index, &(key, _)) in preset.iter().enumerate() {
        let name = key.get();
        if target.contains_key(name) && !order.iter().any(|known| known == name) {
            let slot = preset
                .iter()
                .skip(index)
                .find_map(|(later, _)| order.iter().position(|known| known == later.get()))
                .unwrap_or(order.len());
            if let Some(next) = order.get(slot) {
                hand_over_header(target, key, next);
            }
            order.insert(slot, name.to_owned());
        }
    }
    target.sort_values_by(|one, _, other, _| {
        let rank = |key: &Key| order.iter().position(|known| known == key.get());
        rank(one).cmp(&rank(other))
    });
}

fn hand_over_header(target: &mut Table, key: &Key, next: &str) {
    if let Some(mut following) = target.key_mut(next)
        && following.leaf_decor().prefix() == key.leaf_decor().prefix()
    {
        following.leaf_decor_mut().clear();
    }
}

fn header(key: &Key) -> Option<&str> {
    key.leaf_decor()
        .prefix()?
        .as_str()?
        .lines()
        .find_map(|line| line.strip_prefix("# "))
}

fn rules(table: &Table) -> impl Iterator<Item = Rule<'_>> {
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

fn lints_path(cargo_toml: &DocumentMut) -> &'static [&'static str] {
    if cargo_toml.contains_key("workspace") {
        &["workspace", "lints"]
    } else {
        &["lints"]
    }
}

fn table_at<'doc>(root: &'doc mut Table, path: &[&str]) -> Result<&'doc mut Table> {
    path.iter().try_fold(root, |parent, name| {
        unfold(parent, name);
        parent
            .entry(name)
            .or_insert_with(implicit_table)
            .as_table_mut()
            .with_context(|| format!("`{name}` in Cargo.toml is not a table"))
    })
}

fn unfold(parent: &mut Table, name: &str) {
    let Some(folded) = parent.get_mut(name) else {
        return;
    };
    if !folded.is_inline_table() && !folded.as_table().is_some_and(Table::is_dotted) {
        return;
    }
    if let Ok(mut table) = mem::take(folded).into_table() {
        table.set_dotted(false);
        *folded = Item::Table(table);
    }
    parent.set_implicit(true);
    if let Some(mut key) = parent.key_mut(name) {
        key.leaf_decor_mut().clear();
    }
}

fn implicit_table() -> Item {
    let mut table = Table::new();
    table.set_implicit(true);
    Item::Table(table)
}

pub(crate) fn same(current: &Item, setting: &Item) -> bool {
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
