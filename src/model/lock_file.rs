use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    hash::{Hash, Hasher},
    io::{self, BufReader, BufWriter},
    path::PathBuf,
};
use thiserror::Error;

use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Error, Debug)]
pub enum LockFileError {
    #[error("unable to read lock file")]
    IO(io::Error),
    #[error("unable to deserialize lock file")]
    Decode(serde_json::Error),
    #[error("unable to serialize lock file")]
    Encode(serde_json::Error),
}

#[derive(Debug)]
pub struct LockFile {
    pub path: PathBuf,
    pub dependencies: DependenciesMap,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DependenciesMap(
    #[serde(serialize_with = "sorted_dependencies")] HashMap<DependencyID, DependencyLock>,
);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DependencyID(String, String);

impl From<(String, String)> for DependencyID {
    fn from(info: (String, String)) -> Self {
        Self(info.0, info.1)
    }
}

fn sorted_dependencies<S>(
    value: &HashMap<DependencyID, DependencyLock>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

impl<'de> de::Deserialize<'de> for DependencyID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let mut parts = s.split("@");
        let name = parts
            .next()
            .ok_or_else(|| de::Error::custom("missing dependency name"))?;
        let version = parts
            .next()
            .ok_or_else(|| de::Error::custom("missing dependency version"))?;
        Ok(DependencyID(name.to_string(), version.to_string()))
    }
}

impl ser::Serialize for DependencyID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Serialize::serialize(&format!("{}@{}", self.0, self.1), serializer)
    }
}

impl Hash for DependencyID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&format!("{}@{}", self.0, self.1).as_bytes());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DependencyLock {
    pub name: String,
    pub version: String,
    pub tarball: String,
    pub sha1: String,
}

/// The lock file is responsible for locking/pinning dependency versions in a given project.
/// It stores a list of dependencies along with their resolved version, registry url, and sha1 checksum.
///
/// # Example
///
/// ```
/// // Load the lock file for the current project or create new lock file
/// let mut lock_file = LockFile::load(lock_file_path)
///     .unwrap_or_else(|| LockFile::new(lock_file_path));
///
/// // Add dependency
/// lock_file.add(
///     ("react".to_string(), "^1.0.0".to_string()),
///     DependencyLock {
///         name: "react".to_string(),
///         version: "1.2.6".to_string(),
///         tarbal: String::new(),
///         sha1: String::new(),
///     }
/// );
///
/// // Save changes to disk
/// lock_file.save().expect("Unable to save lock file");
/// ```
impl LockFile {
    /// Creates a new instance of a lock file with a path it should be saved at.
    /// It can be saved to the file by calling [`Self::save()`].
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            dependencies: DependenciesMap(HashMap::with_capacity(1)), // We will be installing at least 1 dependency
        }
    }

    /// Loads a lock file from the given path.
    pub fn load(path: PathBuf) -> Result<Self, LockFileError> {
        let lock_file = File::open(&path).map_err(LockFileError::IO)?;
        let reader = BufReader::new(lock_file);

        Ok(LockFile {
            path,
            dependencies: serde_json::from_reader(reader).map_err(LockFileError::Decode)?,
        })
    }

    /// Saves a lock file to the same path it was opened from.
    pub fn save(&self) -> Result<(), LockFileError> {
        let lock_file = File::create(&self.path).map_err(LockFileError::IO)?;
        let writer = BufWriter::new(lock_file);

        serde_json::to_writer_pretty(writer, &self.dependencies).map_err(LockFileError::Encode)
    }

    /// Add a dependency to the lock file.
    pub fn add<T: Into<DependencyID>>(&mut self, id: T, dep: DependencyLock) {
        self.dependencies.0.insert(id.into(), dep);
    }
}
