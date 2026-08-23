use crate::Result;
use crate::registry::{CURRENT_USER, Key, Node, StringEntry};
use std::ffi::OsStr;
use std::path::PathBuf;

pub struct Environment {
    node: Node,
    path: StringEntry,
}

impl Environment {
    pub fn new() -> Result<Self> {
        Self::create(CURRENT_USER, "Environment", true)
    }

    pub fn create(parent: &Key, path: impl AsRef<str>, is_path_expand: bool) -> Result<Self> {
        Ok(Self {
            node: Node::create(parent, path)?,
            path: StringEntry::new("Path", is_path_expand),
        })
    }

    pub fn path(&self) -> Result<Vec<PathBuf>> {
        self.get_paths(&self.path)
    }

    pub fn set_path(&self, value: Vec<PathBuf>) -> Result<()> {
        self.set_paths(&self.path, value)
    }

    pub fn path_push(&self, item: impl AsRef<OsStr>) -> Result<()> {
        self.paths_push(&self.path, item)
    }

    pub fn path_insert(&self, item: impl AsRef<OsStr>) -> Result<()> {
        self.paths_insert(&self.path, item)
    }

    pub fn path_remove(&self, item: impl AsRef<OsStr>) -> Result<()> {
        self.paths_remove(&self.path, item)
    }
}

impl std::ops::Deref for Environment {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
