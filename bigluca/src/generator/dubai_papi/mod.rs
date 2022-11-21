//! # Dubai papi collection generator
//!
//! Generator for the Dubai papi collection

mod attributes;

use super::GenerateNft;
use crate::{
    config::DubaiPapiConfiguration,
    nft::{Attribute as NftAttribute, Metadata, Nft},
};

/// Dubai papi collection generator
pub struct DubaiPapi {
    config: DubaiPapiConfiguration,
}

impl DubaiPapi {
    /// Instantiates new DubaiPapi from configuration
    pub fn new(config: DubaiPapiConfiguration) -> Self {
        Self { config }
    }
}

impl GenerateNft for DubaiPapi {
    fn generate_nft(&self, seed: Option<String>) -> Nft {
        todo!();
    }
}
