use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, ensure};
use cargo_metadata::MetadataCommand;

use crate::toml_file::TomlFile;

pub(crate) struct Workspace {
    root: PathBuf,
    members: Vec<String>,
}

impl Workspace {
    pub(crate) fn locate(path: &Path) -> Result<Self> {
        ensure!(path.is_dir(), "{} is not a directory", path.display());

        let metadata = MetadataCommand::new()
            .current_dir(path)
            .no_deps()
            .exec()
            .with_context(|| format!("reading the workspace at {}", path.display()))?;
        let root = metadata.workspace_root;
        let root_manifest = root.join("Cargo.toml");

        let members = metadata
            .packages
            .iter()
            .map(|package| &package.manifest_path)
            .filter(|manifest| **manifest != root_manifest)
            .map(|manifest| manifest.strip_prefix(&root).map(ToString::to_string))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            root: root.into(),
            members,
        })
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }

    pub(crate) fn cargo_toml(&self) -> Result<TomlFile> {
        TomlFile::open(&self.root, "Cargo.toml")
    }

    pub(crate) fn members(&self) -> Result<Vec<TomlFile>> {
        self.members
            .iter()
            .map(|member| TomlFile::open(&self.root, member))
            .collect()
    }

    pub(crate) fn clippy_toml(&self) -> Result<TomlFile> {
        let name = if self.root.join(".clippy.toml").is_file() {
            ".clippy.toml"
        } else {
            "clippy.toml"
        };
        TomlFile::open(&self.root, name)
    }
}
