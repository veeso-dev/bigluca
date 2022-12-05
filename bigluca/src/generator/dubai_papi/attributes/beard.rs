use super::HairColor;
use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute, FromAttributes},
    render::{AsLayer, Layer},
};

use std::path::PathBuf;

const TRAIT_TYPE: &str = "Beard";

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
            TRAIT_TYPE,
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

impl FromAttributes for Beard {
    fn from_attributes(attributes: &[Attribute]) -> Option<Self> {
        attributes
            .iter()
            .find(|x| x.trait_type == TRAIT_TYPE)
            .map(|x| match x.value.as_str() {
                "Chevron" => Some(Self::Chevron),
                "Chin Strip" => Some(Self::ChinStrip),
                "Circle Beard" => Some(Self::CircleBeard),
                "Royale Beard" => Some(Self::RoyaleBeard),
                "Short Boxed Beard" => Some(Self::ShortBoxedBeard),
                "Three Day Stubble Beard" => Some(Self::ThreeDayStubbleBeard),
                _ => None,
            })
            .flatten()
    }
}

impl AsLayer<&DubaiPapiConfiguration, HairColor> for Beard {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, states: HairColor) -> anyhow::Result<Layer> {
        Layer::from_path(&self.path(paths, states))
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

    #[test]
    fn should_generate_attribute_with_correct_name() {
        assert_eq!(&Beard::Chevron.as_attribute().trait_type, "Beard");
    }
}
