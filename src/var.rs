use crate::Result;
use crate::env::Environment;
use std::ffi::OsStr;

pub fn set(name: impl AsRef<str>, value: impl AsRef<OsStr>) -> Result<()> {
    Environment::new()?.set(name, value)
}

pub fn set_expand(name: impl AsRef<str>, value: impl AsRef<OsStr>) -> Result<()> {
    Environment::new()?.set_expand(name, value)
}

pub fn remove(name: impl AsRef<str>) -> Result<()> {
    Environment::new()?.remove(name)
}

pub fn placeholder(name: impl AsRef<str>) -> String {
    format!("%{}%", name.as_ref())
}

pub mod path {
    use super::*;

    pub fn push(item: impl AsRef<OsStr>) -> Result<()> {
        Environment::new()?.path_push(item)
    }

    pub fn insert(item: impl AsRef<OsStr>) -> Result<()> {
        Environment::new()?.path_insert(item)
    }

    pub fn remove(item: impl AsRef<OsStr>) -> Result<()> {
        Environment::new()?.path_remove(item)
    }
}
