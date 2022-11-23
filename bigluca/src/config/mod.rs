//! # Configuration

mod dubai_papi;

use std::path::Path;

pub use dubai_papi::DubaiPapiConfiguration;

/// Validate configuration
pub trait Validate {
    fn validate(&self) -> anyhow::Result<()>;
}

/// Bigluca configuration
#[derive(Debug, ValidateAllFields, Clone, PartialEq, Deserialize)]
pub struct Configuration {
    pub dubai_papi: DubaiPapiConfiguration,
}

impl Configuration {
    /// Parse and load configuration from file
    pub fn parse(path: &Path) -> anyhow::Result<Self> {
        crate::utils::serde::deserialize(path)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_parse_and_validate_configuration_from_default() {
        let config = Configuration::parse(Path::new("./test/config.json")).unwrap();
        assert!(config.validate().is_ok());
    }
}
