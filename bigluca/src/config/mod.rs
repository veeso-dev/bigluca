//! # Configuration

mod dubai_papi;

use std::fs;
use std::path::Path;

pub use dubai_papi::DubaiPapiConfiguration;

/// Validate configuration
pub trait Validate {
    fn validate(&self) -> anyhow::Result<()>;
}

/// Bigluca configuration
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Configuration {
    pub dubai_papi: DubaiPapiConfiguration,
}

impl Configuration {
    /// Parse and load configuration from file
    pub fn parse(path: &Path) -> anyhow::Result<Self> {
        let file = fs::File::open(path)?;
        let config = serde_json::from_reader(&file)?;
        Ok(config)
    }
}

impl Validate for Configuration {
    fn validate(&self) -> anyhow::Result<()> {
        todo!();
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_parse_and_validate_configuration_from_default() {
        let config = Configuration::parse(Path::new("./config/config.json")).unwrap();
        assert!(config.validate().is_ok());
    }
}
