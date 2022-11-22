//! # App
//!
//! Main bigluca application

use std::path::PathBuf;

use crate::{
    config::Configuration as EngineConfiguration,
    database::nft::NftDatabase,
    generator::{Collection, GenerateNft, Generator},
};

/// Application
#[derive(Debug)]
pub struct App {
    collection: Collection,
    config: EngineConfiguration,
    database: NftDatabase,
    output_dir: PathBuf,
}

impl App {
    pub fn new(
        collection: Collection,
        config: EngineConfiguration,
        database: NftDatabase,
        output_dir: PathBuf,
    ) -> Self {
        Self {
            collection,
            config,
            database,
            output_dir,
        }
    }

    /// Run application to generate `nfts` new nfts
    pub fn run(mut self, nfts: usize) -> anyhow::Result<()> {
        debug!("running application for {} cycles", nfts);
        for i in 0..nfts {
            info!("minting NFT {} out of {}", i + 1, nfts);
            let nft =
                Generator::new(self.collection, &self.config, &mut self.database).generate_nft()?;
            info!("index: {}", nft.collection_index());
            info!("name: {}", nft.metadata().name);
            info!("description: {}", nft.metadata().description);
            info!("attributes:\n{}", nft.metadata().pretty_attributes());
            let mut output_metadata_file = self.output_dir.clone();
            output_metadata_file.push(format!(
                "{:0width$}-{}.json",
                nft.collection_index(),
                nft.metadata().name.replace(' ', "-"),
                width = 4
            ));
            let mut output_image_file = self.output_dir.clone();
            output_image_file.push(format!(
                "{:0width$}-{}.png",
                nft.collection_index(),
                nft.metadata().name.replace(' ', "-"),
                width = 4
            ));
            debug!("serializing metadata to file");
            crate::utils::serde::serialize(nft.metadata(), &output_metadata_file)?;
            info!("metadata written to {}", output_metadata_file.display());
            debug!("writing image to file");
            nft.image()
                .save_with_format(&output_image_file, image::ImageFormat::Png)?;
            info!("image written to {}", output_image_file.display());
        }
        debug!("terminating application");

        Ok(())
    }
}
