use crate::Result;
use std::{
    env::{join_paths, split_paths},
    ffi::{OsStr, OsString},
    path::PathBuf,
};
use windows::Win32::Foundation::ERROR_FILE_NOT_FOUND;
use windows_registry::{CURRENT_USER, HSTRING, Key};

const PATH: &'static str = "Path";

pub struct Environment {
    key: Key,
    is_path_expand: bool,
}

impl Environment {
    pub fn new() -> Result<Self> {
        Self::create(CURRENT_USER, "Environment", true)
    }

    pub fn create(parent: &Key, path: impl AsRef<str>, is_path_expand: bool) -> Result<Self> {
        Ok(Self {
            key: parent.create(path)?,
            is_path_expand,
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

    pub fn path(&self) -> Result<Vec<PathBuf>> {
        Ok(split_paths(&self.get(PATH)?.unwrap_or_default()).collect())
    }

    pub fn set_path(&self, value: Vec<PathBuf>) -> Result<()> {
        let value = join_paths(value)?;
        match self.is_path_expand {
            true => self.set_expand(PATH, value),
            false => self.set(PATH, value),
        }
    }

    fn path_add(&self, item: impl AsRef<OsStr>, is_insert: bool) -> Result<()> {
        let mut paths = self.path()?;
        for item in split_paths(item.as_ref()) {
            if !paths.iter().any(|p| same_path(p, &item)) {
                match is_insert {
                    true => paths.insert(0, item),
                    false => paths.push(item),
                }
            }
        }
        self.set_path(paths)
    }

    pub fn path_push(&self, item: impl AsRef<OsStr>) -> Result<()> {
        self.path_add(item, false)
    }
    pub fn path_insert(&self, item: impl AsRef<OsStr>) -> Result<()> {
        self.path_add(item, true)
    }

    pub fn path_remove(&self, item: impl AsRef<OsStr>) -> Result<()> {
        let items: Vec<PathBuf> = split_paths(item.as_ref()).collect();
        let mut paths = self.path()?;
        paths.retain(|p| !items.iter().any(|i| same_path(i, p)));
        self.set_path(paths)
    }
}

fn same_path(a: impl AsRef<OsStr>, b: impl AsRef<OsStr>) -> bool {
    a.as_ref().eq_ignore_ascii_case(b)
}
