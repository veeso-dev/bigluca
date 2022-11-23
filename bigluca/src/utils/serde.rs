//! # Serde utils

use serde::{de::DeserializeOwned, Serialize};
use std::fs::OpenOptions;
use std::path::Path;

pub fn serialize<S>(serializable: &S, path: &Path) -> anyhow::Result<()>
where
    S: Serialize + Sized,
{
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    serde_json::to_writer_pretty(&file, serializable)?;

    Ok(())
}

/// ### deserialize
///
/// Read data from readable and deserialize its content as TOML
pub fn deserialize<S>(path: &Path) -> anyhow::Result<S>
where
    S: DeserializeOwned + Sized + std::fmt::Debug,
{
    let file = OpenOptions::new().read(true).open(path)?;
    let data: S = serde_json::from_reader(&file)?;

    Ok(data)
}
