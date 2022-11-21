//! # Nft
//!
//! Types for NFT

use image::DynamicImage;

pub use self::metadata::{Attribute, Metadata};

mod metadata;

#[derive(Debug, Clone, PartialEq)]
pub struct Nft {
    image: DynamicImage,
    metadata: Metadata,
}

impl Nft {
    /// Instantiates a new Nft
    pub fn new(image: DynamicImage, metadata: Metadata) -> Self {
        Self { image, metadata }
    }

    /// Return NFT image
    pub fn image(&self) -> &DynamicImage {
        &self.image
    }

    /// Return NFT metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Return NFT hash
    pub fn hash(&self) -> String {
        todo!()
    }
}
