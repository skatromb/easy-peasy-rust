use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use cargo_metadata::MetadataCommand;

pub(crate) struct Workspace {
    root: PathBuf,
    members: Vec<String>,
}

impl Workspace {
    pub(crate) fn locate(path: &Path) -> Result<Self> {
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

    pub(crate) fn members(&self) -> &[String] {
        &self.members
    }

    pub(crate) fn clippy_toml(&self) -> &'static str {
        if self.root.join(".clippy.toml").is_file() {
            ".clippy.toml"
        } else {
            "clippy.toml"
        }
    }
}
