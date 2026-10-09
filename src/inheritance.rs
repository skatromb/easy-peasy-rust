use std::io::{Write as _, stderr};
use std::iter;

use anyhow::Result;
use toml_edit::{Item, value};

use crate::Choices;
use crate::block::{Block, Kind, Question, shown};
use crate::merge::{Split, same};
use crate::toml_file::TomlFile;

pub(crate) fn merge(
    cargo_toml: &mut TomlFile,
    members: &mut [TomlFile],
    choices: Choices,
) -> Result<()> {
    let preset = Item::Table(iter::once(("workspace", value(true))).collect());
    let doc = cargo_toml.doc();
    let root = (doc.contains_key("workspace") && doc.contains_key("package")).then_some(cargo_toml);
    let (missing, changed) = not_inheriting(root.into_iter().chain(members), &preset);
    let mut block = Block::new(Kind::Inheritance, "Workspace lints");

    for manifest in &missing {
        block.push_name(manifest.name());
    }
    if block.ask(Question::Adopt, choices)? {
        switch(missing, &preset);
    }
    replace_own(&mut block, changed, &preset, choices)
}

fn replace_own(
    block: &mut Block,
    changed: Vec<&mut TomlFile>,
    preset: &Item,
    choices: Choices,
) -> Result<()> {
    for manifest in &changed {
        block.push(manifest.name(), manifest.doc().get("lints"), preset);
    }
    if !block.ask(Question::Replace, choices)? {
        return Ok(());
    }
    if !choices.diff {
        warn_dropped(&changed)?;
    }
    switch(changed, preset);
    Ok(())
}

fn warn_dropped(manifests: &[&mut TomlFile]) -> Result<()> {
    for manifest in manifests {
        let lints = manifest.doc().get("lints").map(shown).unwrap_or_default();
        writeln!(
            stderr(),
            "warning: {} dropped its own lints: {lints}",
            manifest.name()
        )?;
    }
    Ok(())
}

fn switch(manifests: Vec<&mut TomlFile>, preset: &Item) {
    for manifest in manifests {
        manifest.doc_mut()["lints"] = preset.clone();
    }
}

fn not_inheriting<'doc>(
    manifests: impl Iterator<Item = &'doc mut TomlFile>,
    preset: &Item,
) -> Split<&'doc mut TomlFile> {
    manifests
        .filter(|manifest| {
            !manifest
                .doc()
                .get("lints")
                .is_some_and(|lints| same(lints, preset))
        })
        .partition(|manifest| !manifest.doc().contains_key("lints"))
}
