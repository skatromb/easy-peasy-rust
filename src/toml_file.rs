use std::fs;
use std::io::{ErrorKind, Write as _, stdout};
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use toml_edit::DocumentMut;

pub(crate) struct TomlFile {
    name: String,
    path: PathBuf,
    before: Option<String>,
    doc: DocumentMut,
}

impl TomlFile {
    pub(crate) fn open(dir: &Path, name: &str) -> Result<Self> {
        let path = dir.join(name);
        let before = read(&path)?;
        let doc = before
            .as_deref()
            .unwrap_or_default()
            .parse()
            .with_context(|| format!("parsing {name}"))?;

        Ok(Self {
            name: name.to_owned(),
            path,
            before,
            doc,
        })
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn doc(&self) -> &DocumentMut {
        &self.doc
    }

    pub(crate) const fn doc_mut(&mut self) -> &mut DocumentMut {
        &mut self.doc
    }

    pub(crate) fn is_changed(&self) -> bool {
        self.before.as_deref() != Some(self.doc.to_string().as_str())
    }

    pub(crate) fn save(&self) -> Result<()> {
        let after = self.doc.to_string();

        let status = match &self.before {
            None => "Created",
            Some(before) if *before == after => "Unchanged",
            Some(_) => "Updated",
        };

        if status != "Unchanged" {
            fs::write(&self.path, after).with_context(|| format!("writing {}", self.name))?;
        }

        writeln!(stdout(), "{status:>12} {}", self.name)?;
        Ok(())
    }
}

fn read(path: &Path) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("reading {}", path.display())),
    }
}
