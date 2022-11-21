//! # Generator
//!
//! Exposes the different generators for the NFT collections

use crate::nft::Nft;

mod dubai_papi;

pub use dubai_papi::DubaiPapi;

/// The trait `GenerateNft` defines the method `generate_nft` which must be implemented by all the NFT collcetions generator
pub trait GenerateNft {
    fn generate_nft(&self, seed: Option<String>) -> Nft;
}
