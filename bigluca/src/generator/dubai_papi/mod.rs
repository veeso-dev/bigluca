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

const MAX_MINT: usize = 1000;
/// Dubai papi collection generator
pub struct DubaiPapi<'a> {
    config: &'a DubaiPapiConfiguration,
    database: &'a mut NftDatabase,
}

impl<'a> DubaiPapi<'a> {
    /// Instantiates new DubaiPapi from configuration
    pub fn new(config: &'a DubaiPapiConfiguration, database: &'a mut NftDatabase) -> Self {
        Self { config, database }
    }
}

impl<'a> GenerateNft for DubaiPapi<'a> {
    fn generate_nft(self) -> anyhow::Result<Nft> {
        if self.database.dubai_papi_hash.len() >= MAX_MINT {
            anyhow::bail!("cannot mint NFT: MAX MINT ({}) reached", MAX_MINT);
        }
        todo!();
    }
}
