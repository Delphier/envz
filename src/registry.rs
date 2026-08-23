use crate::Result;
use std::env::{join_paths, split_paths};
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use windows::Win32::Foundation::ERROR_FILE_NOT_FOUND;
use windows_registry::HSTRING;
pub use windows_registry::{CURRENT_USER, Key, LOCAL_MACHINE};

pub struct StringEntry {
    pub name: &'static str,
    pub is_expand: bool,
}

impl StringEntry {
    pub fn new(name: &'static str, is_expand: bool) -> Self {
        Self { name, is_expand }
    }
}

pub struct Node {
    key: Key,
}

impl Node {
    pub fn create(parent: &Key, path: impl AsRef<str>) -> Result<Self> {
        Ok(Self {
            key: parent.create(path)?,
        })
    }

    pub fn set(&self, name: impl AsRef<str>, value: impl AsRef<OsStr>) -> Result<()> {
        self.set_string(name, value, false)
    }

    pub fn set_expand(&self, name: impl AsRef<str>, value: impl AsRef<OsStr>) -> Result<()> {
        self.set_string(name, value, true)
    }

    pub fn remove(&self, name: impl AsRef<str>) -> Result<()> {
        self.remove_value(name)
    }

    fn set_string(
        &self,
        name: impl AsRef<str>,
        value: impl AsRef<OsStr>,
        is_expand: bool,
    ) -> Result<()> {
        let value = &HSTRING::from(value.as_ref());
        Ok(match is_expand {
            true => self.key.set_expand_hstring(name, value)?,
            false => self.key.set_hstring(name, value)?,
        })
    }

    fn remove_value(&self, name: impl AsRef<str>) -> Result<()> {
        Ok(match self.key.remove_value(name) {
            Err(e) if e.code() == ERROR_FILE_NOT_FOUND.to_hresult() => (),
            r @ _ => r?,
        })
    }

    pub fn get(&self, name: impl AsRef<str>) -> Result<Option<OsString>> {
        match self.key.get_hstring(name) {
            Ok(s) => Ok(Some(s.to_os_string())),
            Err(e) if e.code() == ERROR_FILE_NOT_FOUND.to_hresult() => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn get_paths(&self, entry: &StringEntry) -> Result<Vec<PathBuf>> {
        Ok(split_paths(&self.get(entry.name)?.unwrap_or_default()).collect())
    }

    pub fn set_paths(&self, entry: &StringEntry, value: Vec<PathBuf>) -> Result<()> {
        let value = join_paths(value)?;
        match entry.is_expand {
            true => self.set_expand(entry.name, value),
            false => self.set(entry.name, value),
        }
    }

    fn paths_add(
        &self,
        entry: &StringEntry,
        item: impl AsRef<OsStr>,
        is_insert: bool,
    ) -> Result<()> {
        let mut paths = self.get_paths(entry)?;
        for item in split_paths(item.as_ref()) {
            if !paths.iter().any(|p| same_path(p, &item)) {
                match is_insert {
                    true => paths.insert(0, item),
                    false => paths.push(item),
                }
            }
        }
        self.set_paths(entry, paths)
    }

    pub fn paths_push(&self, entry: &StringEntry, item: impl AsRef<OsStr>) -> Result<()> {
        self.paths_add(entry, item, false)
    }
    pub fn paths_insert(&self, entry: &StringEntry, item: impl AsRef<OsStr>) -> Result<()> {
        self.paths_add(entry, item, true)
    }

    pub fn paths_remove(&self, entry: &StringEntry, item: impl AsRef<OsStr>) -> Result<()> {
        let items: Vec<PathBuf> = split_paths(item.as_ref()).collect();
        let mut paths = self.get_paths(entry)?;
        paths.retain(|p| !items.iter().any(|i| same_path(i, p)));
        self.set_paths(entry, paths)
    }
}

fn same_path(a: impl AsRef<OsStr>, b: impl AsRef<OsStr>) -> bool {
    a.as_ref().eq_ignore_ascii_case(b)
}
