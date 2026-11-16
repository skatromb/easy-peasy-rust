use std::collections::BTreeSet;
use std::mem;

use anyhow::{Context as _, Result};
use toml_edit::{DocumentMut, Item, Key, KeyMut, RawString, Table, Value};

use crate::Choices;
use crate::block::{self, Block, Kind, Question};
use crate::toolchain::Toolchain;

const LINTS: &str = include_str!("../rules/lints.toml");
const SETTINGS: &str = include_str!("../rules/clippy.toml");
const TOOLS: [Kind; 4] = [
    Kind::RustcLint,
    Kind::ClippyLint,
    Kind::RustdocLint,
    Kind::CargoLint,
];
const MARK: &str = "# easy-peasy:";
const YOURS: &str = "yours";

type Rule<'preset> = (&'preset Key, &'preset Item);
type Setting<'preset> = (&'preset Key, Item);
pub(crate) type Split<T> = (Vec<T>, Vec<T>);

pub(crate) fn settings(
    clippy_toml: &mut DocumentMut,
    choices: Choices,
    toolchain: &Toolchain,
) -> Result<Vec<String>> {
    let all: DocumentMut = SETTINGS.parse()?;
    let mut known = all.as_table().clone();
    let skipped = toolchain.retain(&mut known, Kind::ClippySetting);

    remove_old(clippy_toml, &all, Kind::ClippySetting, choices)?;
    report_layout(clippy_toml, &all, Kind::ClippySetting)?;
    table(clippy_toml, &known, Kind::ClippySetting, choices)?;
    lay_out(clippy_toml, &all);
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
        remove_old(yours, all, kind, choices)?;
        report_layout(yours, all, kind)?;
        table(yours, &known, kind, choices)?;
        remove_yours(yours, all, kind, choices)?;
        lay_out(yours, all);
    }
    Ok(skipped)
}

fn remove_yours(target: &mut Table, preset: &Table, kind: Kind, choices: Choices) -> Result<()> {
    let old = old_names(target, preset);
    let yours: BTreeSet<String> = target
        .iter()
        .map(|(name, _)| name.to_owned())
        .filter(|name| !preset.contains_key(name) && !old.contains(name))
        .collect();
    let title = format!("Your {} lints not in `easy-peasy-rust`", kind.section());
    if removal(target, &yours, Block::new(kind, &title)).ask(Question::RemoveYours, choices)? {
        target.retain(|name, _| !yours.contains(name));
    }
    Ok(())
}

fn remove_old(target: &mut Table, preset: &Table, kind: Kind, choices: Choices) -> Result<()> {
    let old = old_names(target, preset);
    let title = format!("Old `easy-peasy-rust` {}", kind.what());
    if removal(target, &old, Block::new(kind, &title)).ask(Question::RemoveOld, choices)? {
        target.retain(|name, _| !old.contains(name));
    }
    Ok(())
}

fn removal(target: &Table, names: &BTreeSet<String>, mut block: Block) -> Block {
    for name in names {
        block.push(name, target.get(name), None);
    }
    block
}

fn old_names(target: &Table, preset: &Table) -> BTreeSet<String> {
    let mut in_preset_block = false;
    rules(target)
        .filter(|&(key, setting)| {
            let lines = line_prefix(key, setting).trim_end_matches([' ', '\t']);
            in_preset_block = lines.lines().fold(in_preset_block, |inside, line| {
                title(line).map_or_else(|| inside && !line.trim().is_empty(), |name| name != YOURS)
            });
            in_preset_block && !preset.contains_key(key.get())
        })
        .map(|(key, _)| key.get().to_owned())
        .collect()
}

fn table(target: &mut Table, preset: &Table, kind: Kind, choices: Choices) -> Result<()> {
    let wanted: Vec<_> = rules(preset).collect();
    blocks(&wanted).try_for_each(|block| adopt(target, block, kind, choices))
}

fn report_layout(target: &Table, preset: &Table, kind: Kind) -> Result<()> {
    let mut laid = target.clone();
    lay_out(&mut laid, preset);
    if laid.to_string() != target.to_string() {
        block::rearranged(kind)?;
    }
    Ok(())
}

fn lay_out(target: &mut Table, preset: &Table) {
    let wanted: Vec<_> = rules(preset).collect();
    let rank = |name: &str| {
        wanted
            .iter()
            .position(|(rule, _)| rule.get() == name)
            .unwrap_or(usize::MAX)
    };
    let printed = |key: &Key, setting: &Item| (rank(key.get()), is_sub_table(setting));
    target.sort_values_by(|one, first, other, second| {
        printed(one, first).cmp(&printed(other, second))
    });
    let gap = title_blocks(target, &wanted);
    let own: Vec<String> = target
        .iter()
        .map(|(name, _)| name.to_owned())
        .filter(|name| rank(name) == usize::MAX)
        .collect();
    unmark(target, &own, gap);
}

fn unmark(target: &mut Table, own: &[String], gap: &str) {
    for (index, name) in own.iter().enumerate() {
        let comments = target
            .get_key_value(name)
            .map(|(key, setting)| unmarked(line_prefix(key, setting)))
            .unwrap_or_default();
        let prefix = if index == 0 {
            format!("{gap}{}", comments.trim_start())
        } else {
            comments
        };
        if let Some((key, setting)) = target.get_key_value_mut(name) {
            set_line_prefix(key, setting, prefix);
        }
    }
}

fn is_sub_table(setting: &Item) -> bool {
    setting.as_table().is_some_and(|table| !table.is_dotted())
}

fn title_blocks(target: &mut Table, preset: &[Rule<'_>]) -> &'static str {
    let mut gap = "";
    for block in blocks(preset) {
        let title = block.first().and_then(|&(key, _)| header(key));
        let mut prefix = format!("{gap}{MARK} {}\n", title.unwrap_or_default());
        for &(key, _) in block {
            let inline = target
                .get_key_value_mut(key.get())
                .filter(|(_, setting)| !is_sub_table(setting));
            if let Some((mine, setting)) = inline {
                set_line_prefix(mine, setting, mem::take(&mut prefix));
                gap = "\n";
            }
        }
    }
    gap
}

fn unmarked(prefix: &str) -> String {
    prefix
        .split_inclusive('\n')
        .filter(|line| title(line).is_none())
        .collect()
}

fn line_prefix<'table>(key: &'table Key, setting: &'table Item) -> &'table str {
    let decor = match setting {
        Item::Table(dotted) if dotted.is_dotted() => {
            let first = rules(dotted).next();
            return first.map_or("", |(inner, nested)| line_prefix(inner, nested));
        }
        Item::Table(table) => table.decor(),
        Item::ArrayOfTables(tables) => tables
            .iter()
            .next()
            .map_or_else(|| key.leaf_decor(), Table::decor),
        Item::None | Item::Value(_) => key.leaf_decor(),
    };
    decor
        .prefix()
        .and_then(RawString::as_str)
        .unwrap_or_default()
}

fn set_line_prefix(mut key: KeyMut<'_>, setting: &mut Item, prefix: String) {
    match setting {
        Item::Table(dotted) if dotted.is_dotted() => {
            if let Some((inner, nested)) = dotted.iter_mut().next() {
                set_line_prefix(inner, nested, prefix);
            }
        }
        Item::Table(table) => table.decor_mut().set_prefix(prefix),
        Item::ArrayOfTables(tables) => {
            if let Some(first) = tables.iter_mut().next() {
                first.decor_mut().set_prefix(prefix);
            }
        }
        Item::None | Item::Value(_) => key.leaf_decor_mut().set_prefix(prefix),
    }
}

fn blocks<'preset>(
    preset: &'preset [Rule<'preset>],
) -> impl Iterator<Item = &'preset [Rule<'preset>]> {
    preset.chunk_by(|_, &(key, _)| header(key).is_none())
}

fn adopt(target: &mut Table, preset: &[Rule<'_>], kind: Kind, choices: Choices) -> Result<()> {
    let title = preset.first().and_then(|&(key, _)| header(key));
    let mut block = Block::new(kind, title.unwrap_or_default());
    let (missing, changed) = differing(target, preset);

    for (settings, question) in [(missing, Question::Adopt), (changed, Question::Replace)] {
        for (key, setting) in &settings {
            block.push(key.get(), target.get(key.get()), Some(setting));
        }
        if block.ask(question, choices)? {
            for (key, setting) in settings {
                let _ = target.insert(key.get(), setting);
            }
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

fn header(key: &Key) -> Option<&str> {
    key.leaf_decor().prefix()?.as_str()?.lines().find_map(title)
}

fn title(line: &str) -> Option<&str> {
    line.trim_start().strip_prefix(MARK).map(str::trim)
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
