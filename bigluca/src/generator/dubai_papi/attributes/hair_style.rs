use std::path::PathBuf;

use super::HairColor;
use crate::{
    config::DubaiPapiConfiguration,
    nft::{Attribute, IntoAttribute},
    render::{AsLayer, Layer},
};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum HairStyle {
    Bald,
    BobCut,
}

impl HairStyle {
    fn path(&self, paths: &DubaiPapiConfiguration, color: HairColor) -> PathBuf {
        let mut path = match self {
            Self::Bald => return paths.assets.hair_style.bald.clone(),
            Self::BobCut => &paths.assets.hair_style.bob_cut,
        }
        .to_path_buf();
        path.push(match color {
            HairColor::Black => &paths.assets.hair_style.hair_color.black,
            HairColor::Brown => &paths.assets.hair_style.hair_color.brown,
            HairColor::Blonde => &paths.assets.hair_style.hair_color.blonde,
            HairColor::Blue => &paths.assets.hair_style.hair_color.blue,
            HairColor::Green => &paths.assets.hair_style.hair_color.green,
            HairColor::Pink => &paths.assets.hair_style.hair_color.pink,
            HairColor::Red => &paths.assets.hair_style.hair_color.red,
        });
        path
    }
}

impl IntoAttribute for HairStyle {
    fn into_attribute(&self) -> Attribute {
        Attribute::new(
            "Hair Style",
            match self {
                Self::Bald => "Bald",
                Self::BobCut => "Bob Cut",
            },
        )
    }
}

impl AsLayer<&DubaiPapiConfiguration, HairColor> for HairStyle {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, states: HairColor) -> anyhow::Result<Layer> {
        Layer::from_path(&self.path(paths, states), 150, 40)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(HairStyle::all(), &[HairStyle::Bald, HairStyle::BobCut])
    }
}
