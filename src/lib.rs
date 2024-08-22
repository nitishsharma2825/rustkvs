use std::{error::Error, fmt::Display, path::PathBuf};

#[derive(Debug)]
pub enum MyError {
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

impl Error for MyError {}

pub type Result<T> = std::result::Result<T, MyError>;

pub struct KvStore {
}

impl KvStore {

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        Ok(KvStore {})
    }
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        panic!("not implemented")
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        panic!("not implemented")
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        panic!("not implemented")
    }
}