//! # Nft
//!
//! Types for NFT

use image::DynamicImage;

pub use self::metadata::{AsAttribute, Attribute, Metadata};

mod metadata;

#[derive(Debug, Clone, PartialEq)]
pub struct Nft {
    collection_index: usize,
    image: DynamicImage,
    metadata: Metadata,
}

impl Nft {
    /// Instantiates a new Nft
    pub fn new(collection_index: usize, image: DynamicImage, metadata: Metadata) -> Self {
        Self {
            collection_index,
            image,
            metadata,
        }
    }

    /// Return collection index
    pub fn collection_index(&self) -> usize {
        self.collection_index
    }

    /// Return NFT image
    pub fn image(&self) -> &DynamicImage {
        &self.image
    }

    /// Return NFT metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}
