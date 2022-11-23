//! # Layer
//!
//! Image layer entity and traits

use image::DynamicImage;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// The layer defines a layer to render onto the current rendering image
#[derive(Debug)]
pub struct Layer {
    /// Image data
    pub image: DynamicImage,
    /// X position
    pub x: u32,
    /// Y position
    pub y: u32,
}

impl Layer {
    /// Create layer loading an image from path; then define also positioning
    pub fn from_path(path: &Path, x: u32, y: u32) -> anyhow::Result<Self> {
        debug!("loading layer from path {}", path.display());
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        debug!("file opened");
        let image = image::load(&mut reader, image::ImageFormat::Png)?;
        debug!("image loaded in memory");
        Ok(Self { image, x, y })
    }
}

/// the AsLayer trait is required for image parts to create layers from a certain entity to load as layer in
/// image rendering.
/// The `paths` entity can be used to retrieve paths for the provided entity
pub trait AsLayer<T, U> {
    fn as_layer(&self, paths: T, states: U) -> anyhow::Result<Option<Layer>>;
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_load_layer_from_path() {
        let layer = Layer::from_path(Path::new("./test/eth.png"), 32, 48).unwrap();
        assert_eq!(layer.x, 32);
        assert_eq!(layer.y, 48);
        assert_eq!(layer.image.width(), 256);
    }
}
