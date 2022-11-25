//! # Render
//!
//! Module which exposes the types for nft image rendering

mod layer;

use image::{imageops, DynamicImage, GenericImage, GenericImageView, ImageBuffer, RgbaImage};
pub use layer::{AsLayer, Layer};

/// The engine which takes care of rendering layers into image
pub struct RenderEngine;

impl RenderEngine {
    /// Render layers into image with provided sizes
    pub fn render(width: u32, height: u32, layers: Vec<Layer>) -> anyhow::Result<DynamicImage> {
        debug!("rendering an image with size {}x{}", width, height);
        let mut image: RgbaImage =
            ImageBuffer::from_fn(width, height, |_, _| image::Rgba([0, 0, 0, 0]));
        // put layers
        for layer in layers.into_iter() {
            let safe_width = std::cmp::min(width - layer.x, layer.image.width());
            let safe_height = std::cmp::min(height - layer.y, layer.image.height());
            debug!(
                "placing layer at x1:{},y1:{},x2:{},y2:{}",
                layer.x,
                layer.y,
                layer.x + safe_width,
                layer.y + safe_height
            );
            let mut layer_area =
                imageops::crop(&mut image, layer.x, layer.y, safe_width, safe_height);
            // copy image
            for y in 0..safe_height {
                for x in 0..safe_width {
                    let pixel = layer.image.get_pixel(x, y);
                    // if not transparent, copy
                    if pixel.0[3] != 0 {
                        layer_area.put_pixel(x, y, pixel);
                    }
                }
            }
        }

        Ok(image.into())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use image::ImageFormat;
    use log::LevelFilter;
    use pretty_assertions::assert_eq;
    use std::path::{Path, PathBuf};

    #[test]
    fn should_render_empty_image() {
        let _ = env_logger::builder()
            .filter_level(LevelFilter::Trace)
            .is_test(true)
            .try_init();
        let image = RenderEngine::render(350, 350, vec![]).unwrap();
        assert_eq!(image.height(), 350);
        assert_eq!(image.width(), 350);
        output_image(image);
    }

    #[test]
    fn should_render_layers() {
        let _ = env_logger::builder()
            .filter_level(LevelFilter::Trace)
            .is_test(true)
            .try_init();
        let image = RenderEngine::render(
            350,
            350,
            vec![
                Layer::from_path(Path::new("./test/bruno_strati.png")).unwrap(),
                Layer::from_path_with_position(Path::new("./test/eth.png"), 64, 72).unwrap(),
                Layer::from_path_with_position(Path::new("./test/wow.png"), 200, 180).unwrap(),
            ],
        )
        .unwrap();
        assert_eq!(image.height(), 350);
        assert_eq!(image.width(), 350);
        output_image(image);
    }

    fn output_image(image: DynamicImage) {
        let test_image_path = std::env::var("TEST_IMAGE_PATH").ok();
        if let Some(output_path) = test_image_path {
            let mut output_path = PathBuf::from(output_path);
            output_path.push(format!(
                "{}.png",
                crate::utils::random::Random::default().random_alphanumeric_with_len(8)
            ));
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&output_path)
                .unwrap();
            assert!(image.write_to(&mut file, ImageFormat::Png).is_ok());
        }
    }
}
