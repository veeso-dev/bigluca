//! # Configuration

mod dubai_papi;

pub use dubai_papi::DubaiPapiConfiguration;

/// Bigluca configuration
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Configuration {
    pub dubai_papi: DubaiPapiConfiguration,
}
