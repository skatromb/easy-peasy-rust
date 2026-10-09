use std::collections::HashSet;

use anyhow::{Context as _, Result};
use cargo_metadata::semver::Version;
use toml_edit::{DocumentMut, Table};

use crate::block::Kind;

const VALIDITY: &str = include_str!("../rules/validity.toml");

pub(crate) struct Supported {
    entries: HashSet<String>,
}

impl Supported {
    pub(crate) fn new(rust: &Version) -> Result<Self> {
        let validity: DocumentMut = VALIDITY.parse()?;
        let mut entries = HashSet::new();

        for (section, releases) in validity.iter() {
            let names = releases
                .as_table()
                .context("validity section is not a table")?;

            for (name, release) in names {
                let since = release.as_str().context("release is not a string")?;
                let known = Version::parse(&format!("{since}.0"))? <= *rust;
                entries.extend(known.then(|| format!("{section}.{name}")));
            }
        }

        Ok(Self { entries })
    }

    pub(crate) fn retain(&self, preset: &mut Table, kind: Kind) -> Vec<String> {
        let mut skipped = Vec::new();
        preset.retain(|name, _| {
            let known = self.entries.contains(&format!("{}.{name}", kind.section()));
            if !known {
                skipped.push(kind.label(name));
            }
            known
        });
        skipped
    }
}
