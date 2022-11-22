//! # Dubai papi collection generator
//!
//! Generator for the Dubai papi collection

mod attributes;

use super::GenerateNft;
use crate::{
    config::DubaiPapiConfiguration,
    database::nft::NftDatabase,
    nft::{Attribute as NftAttribute, Metadata, Nft},
};

/// Dubai papi collection generator
pub struct DubaiPapi<'a> {
    config: DubaiPapiConfiguration,
    database: &'a mut NftDatabase,
}

impl<'a> DubaiPapi<'a> {
    /// Instantiates new DubaiPapi from configuration
    pub fn new(config: DubaiPapiConfiguration, database: &'a mut NftDatabase) -> Self {
        Self { config, database }
    }
}

impl<'a> GenerateNft for DubaiPapi<'a> {
    fn generate_nft(self) -> anyhow::Result<Nft> {
        todo!();
    }
}
