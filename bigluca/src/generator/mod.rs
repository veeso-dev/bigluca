//! # Generator
//!
//! Exposes the different generators for the NFT collections

use crate::{config::Configuration, database::nft::NftDatabase, nft::Nft};

mod collection;
mod dubai_papi;

pub use collection::Collection;
pub use dubai_papi::DubaiPapi;

/// The trait `GenerateNft` defines the method `generate_nft` which must be implemented by all the NFT collcetions generator
pub trait GenerateNft {
    fn generate_nft(self) -> anyhow::Result<Nft>;
}

/// NFT generator
pub struct Generator<'a> {
    collection: Collection,
    config: Configuration,
    database: &'a mut NftDatabase,
}

impl<'a> Generator<'a> {
    /// Instantiates a new Generator
    pub fn new(
        config: Configuration,
        collection: Collection,
        database: &'a mut NftDatabase,
    ) -> Self {
        Self {
            collection,
            config,
            database,
        }
    }
}

impl<'a> GenerateNft for Generator<'a> {
    /// Generate NFT for provide collection name
    fn generate_nft(self) -> anyhow::Result<Nft> {
        match self.collection {
            Collection::DubaiPapi => {
                DubaiPapi::new(self.config.dubai_papi, self.database).generate_nft()
            }
        }
    }
}
