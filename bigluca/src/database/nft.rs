//! # NFT database
//!
//! This database contains the records for the NFT previously minted

use std::fs;
use std::path::Path;

/// NFT database
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct NftDatabase {
    /// Hash for dubai papi NFTs already recorded
    pub dubai_papi_hash: Vec<String>,
    /// Names assigned to NFTs
    pub names: Vec<String>,
}

impl NftDatabase {
    /// Load NFT database
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        crate::utils::serde::deserialize(path)
    }

    /// Commit changes to file
    pub fn commit(&self, path: &Path) -> anyhow::Result<()> {
        crate::utils::serde::serialize(self, path)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn should_load_and_write_database() {
        let temp = NamedTempFile::new().unwrap();
        write_db(temp.path());
        let mut db = NftDatabase::load(temp.path()).unwrap();
        assert_eq!(db.dubai_papi_hash.len(), 2);
        assert_eq!(db.names.len(), 2);
        db.names.push(String::from("Small Omar"));
        assert!(db.commit(temp.path()).is_ok());

        let db = NftDatabase::load(temp.path()).unwrap();
        assert_eq!(db.dubai_papi_hash.len(), 2);
        assert_eq!(db.names.len(), 3);
    }

    #[test]
    fn should_init_default_db() {
        let db = NftDatabase::default();
        assert_eq!(db.dubai_papi_hash.len(), 0);
        assert_eq!(db.names.len(), 0);
    }

    fn write_db(path: &Path) {
        let mut f = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .unwrap();
        write!(f, "{}", DEFAULT_DB).unwrap();
    }

    const DEFAULT_DB: &str = r##"{
    "dubai_papi_hash": [
      "408c8425-4204-4f03-aa99-c21276ab337f",
      "715d0619-e119-4142-91c3-50cd2763a271"
    ],
    "names": [
      "Big Luca",
      "Smart Francesco"
    ]
  }
"##;
}
