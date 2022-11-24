use super::HairColor;
use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute},
    render::{AsLayer, Layer},
};

use std::path::PathBuf;

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Beard {
    Chevron,
    ChinStrip,
    CircleBeard,
    RoyaleBeard,
    ShortBoxedBeard,
    ThreeDayStubbleBeard,
}

impl Beard {
    fn path(&self, paths: &DubaiPapiConfiguration, color: HairColor) -> PathBuf {
        let mut path = match self {
            Self::Chevron => &paths.assets.beard.chevron,
            Self::ChinStrip => &paths.assets.beard.chin_strip,
            Self::CircleBeard => &paths.assets.beard.circle_beard,
            Self::RoyaleBeard => &paths.assets.beard.royale_beard,
            Self::ShortBoxedBeard => &paths.assets.beard.short_boxed_beard,
            Self::ThreeDayStubbleBeard => &paths.assets.beard.three_day_stubble_beard,
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

impl AsAttribute for Beard {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            "Beard",
            match self {
                Self::Chevron => "Chevron",
                Self::ChinStrip => "Chin Strip",
                Self::CircleBeard => "Circle Beard",
                Self::RoyaleBeard => "Royale Beard",
                Self::ShortBoxedBeard => "Short Boxed Beard",
                Self::ThreeDayStubbleBeard => "Three Day Stubble Beard",
            },
        )
    }
}

impl AsLayer<&DubaiPapiConfiguration, HairColor> for Beard {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, states: HairColor) -> anyhow::Result<Layer> {
        Layer::from_path(&self.path(paths, states), 0, 0)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(
            Beard::all(),
            &[
                Beard::Chevron,
                Beard::ChinStrip,
                Beard::CircleBeard,
                Beard::RoyaleBeard,
                Beard::ShortBoxedBeard,
                Beard::ThreeDayStubbleBeard,
            ]
        )
    }
}
