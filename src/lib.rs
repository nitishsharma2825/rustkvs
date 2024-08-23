use std::{collections::HashMap, error::Error, fmt::Display, fs::{read, File, OpenOptions}, io::{self, BufRead, BufReader, Seek, Write}, path::PathBuf};
use clap::builder::Str;
use serde::{de::value, Deserialize, Serialize};

#[derive(Debug)]
pub enum MyError {
    Io(io::Error),
    Serialize(serde_json::Error),
    Other(String)
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO Error: {}", e),
            MyError::Serialize(e) => write!(f, "Serialization Error: {}", e),
            MyError::Other(e) => write!(f, "Other Error: {}", e),
        }
    }
}

impl Error for MyError {}

impl From<io::Error> for MyError {
    fn from(value: io::Error) -> Self {
        MyError::Io(value)
    }
}

impl From<serde_json::Error> for MyError {
    fn from(value: serde_json::Error) -> Self {
        MyError::Serialize(value)
    }
}

pub type Result<T> = std::result::Result<T, MyError>;

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandType {
    Set,
    Rm
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogValue {
    command_type: CommandType,
    key: String,
    value: Option<String>
}

pub struct KvStore {
    store: HashMap<String, u64>,
    log_file: File,
    current_offset: u64
}

impl KvStore {

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut log_file_path = path.into();
        log_file_path.push("log.txt");

        let log_file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&log_file_path)?;

        Ok(KvStore {
            store : HashMap::new(),
            log_file : log_file,
            current_offset: 0
        })
    }
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let log_value = LogValue {
            command_type: CommandType::Set,
            key: key.clone(),
            value: Some(value)
        };

        let serialized_log_value = serde_json::to_string(&log_value)?;

        self.store.insert(key, self.current_offset);

        writeln!(self.log_file, "{}", serialized_log_value)?;
        self.log_file.flush()?;
        Ok(())
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(offset) = self.store.get(&key) {
            self.log_file.seek(io::SeekFrom::Start(*offset))?;

            let mut buffer = String::new();
            let mut reader = BufReader::new(&self.log_file);
            reader.read_line(&mut buffer)?;

            if buffer.is_empty() {
                return Ok(None);
            }

            let log_value: LogValue = serde_json::from_str(&buffer)?;

            if let Some(value) = log_value.value {
                return Ok(Some(value));
            } else {
                return Ok(None);
            }
        } else {
            Err(MyError::Other("Key not found".to_string()))
        }
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        // Check if the key exists in the map
        if let Some(offset) = self.store.remove(&key) {
            let log_value = LogValue {
                command_type: CommandType::Rm,
                key: key,
                value: None,
            };

            let serialized = serde_json::to_string(&log_value)?;

            // Write the serialized data to the log file
            writeln!(self.log_file, "{}", serialized)?;
            self.log_file.flush()?;
            Ok(())
        } else {
            Err(MyError::Other("Key not found".to_string()))
        }
    }
}