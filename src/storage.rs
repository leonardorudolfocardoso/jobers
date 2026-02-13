use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub trait Storable: Serialize + for<'de> Deserialize<'de> + Default {
    /// Return the filename for this type's storage (e.g., "jobs.json")
    fn storage_filename() -> &'static str;
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Could not determine home directory")]
    HomeNotFound,
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, StorageError>;

/// Returns the storage directory path (~/.jobers/)
fn storage_dir() -> Result<PathBuf> {
    dirs::home_dir()
        .map(|home| home.join(".jobers"))
        .ok_or(StorageError::HomeNotFound)
}

/// Returns the path to the storage file for type T
fn storage_path<T: Storable>() -> Result<PathBuf> {
    storage_dir().map(|dir| dir.join(T::storage_filename()))
}

/// Ensures a directory exists, creating it if necessary
fn ensure_dir(path: &Path) -> Result<()> {
    (!path.exists())
        .then(|| fs::create_dir_all(path))
        .transpose()
        .map(|_| ())
        .map_err(StorageError::from)
}

/// Reads and parses the storage file, returning default if file doesn't exist
fn read_store<T: Storable>(path: &Path) -> Result<T> {
    if !path.exists() {
        return Ok(T::default());
    }

    fs::read_to_string(path)
        .map_err(StorageError::from)
        .and_then(|contents| serde_json::from_str(&contents).map_err(StorageError::from))
}

/// Writes data to disk with pretty formatting
fn write_store<T: Storable>(path: &Path, data: &T) -> Result<()> {
    serde_json::to_string_pretty(data)
        .map_err(StorageError::from)
        .and_then(|json| fs::write(path, json).map_err(StorageError::from))
}

/// Loads data from storage
pub fn load<T: Storable>() -> Result<T> {
    storage_path::<T>().and_then(|path| read_store(&path))
}

/// Saves data to storage, ensuring directory exists
pub fn save<T: Storable>(data: &T) -> Result<()> {
    storage_dir()
        .and_then(|dir| ensure_dir(&dir).map(|_| dir))
        .and_then(|_| storage_path::<T>())
        .and_then(|path| write_store(&path, data))
}

#[cfg(test)]
mod tests {
    use super::{Storable, StorageError};
    use serde::{Deserialize, Serialize};
    use std::io;

    #[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
    struct TestData {
        value: String,
    }

    impl Storable for TestData {
        fn storage_filename() -> &'static str {
            "test.json"
        }
    }

    // Note: These tests are for internal storage functions that are private.
    // We'll focus on testing the public API (load/save) through integration tests.

    #[test]
    fn test_storage_error_display() {
        let err = StorageError::HomeNotFound;
        assert_eq!(err.to_string(), "Could not determine home directory");

        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err = StorageError::Io(io_err);
        assert!(err.to_string().contains("IO error"));

        let json_err = serde_json::from_str::<TestData>("invalid").unwrap_err();
        let err = StorageError::Serialization(json_err);
        assert!(err.to_string().contains("Serialization error"));
    }

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let storage_err: StorageError = io_err.into();
        assert!(matches!(storage_err, StorageError::Io(_)));
    }

    #[test]
    fn test_from_serde_error() {
        let json_err = serde_json::from_str::<TestData>("{}bad").unwrap_err();
        let storage_err: StorageError = json_err.into();
        assert!(matches!(storage_err, StorageError::Serialization(_)));
    }
}
