use std::path::PathBuf;

use super::HairColor;
use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute},
    render::{AsLayer, Layer},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HairStyle {
    // female styles
    Bob,
    Curly,
    Long,
    PonyTail,
    // male styles
    Bald,
    Bun,
    IvyLeague,
    Pixie,
    Taper,
}

impl HairStyle {
    pub fn female() -> &'static [Self] {
        &[Self::Bob, Self::Curly, Self::Long, Self::PonyTail]
    }

    pub fn male() -> &'static [Self] {
        &[
            Self::Bald,
            Self::Bun,
            Self::IvyLeague,
            Self::Pixie,
            Self::Taper,
        ]
    }

    fn path(&self, paths: &DubaiPapiConfiguration, color: HairColor) -> PathBuf {
        let mut path = match self {
            Self::Bald => return paths.assets.hair_style.bald.clone(),
            Self::Bob => &paths.assets.hair_style.bob,
            Self::Curly => &paths.assets.hair_style.curly,
            Self::Long => &paths.assets.hair_style.long,
            Self::PonyTail => &paths.assets.hair_style.pony_tail,
            Self::Bun => &paths.assets.hair_style.bun,
            Self::IvyLeague => &paths.assets.hair_style.ivy_league,
            Self::Pixie => &paths.assets.hair_style.pixie,
            Self::Taper => &paths.assets.hair_style.taper,
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

impl AsAttribute for HairStyle {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            "Hair Style",
            match self {
                Self::Bob => "Bob",
                Self::Curly => "Curly",
                Self::Long => "Long",
                Self::PonyTail => "Pony Tail",
                Self::Bald => "Bald",
                Self::Bun => "Bun",
                Self::IvyLeague => "Ivy League",
                Self::Pixie => "Pixie",
                Self::Taper => "Taper",
            },
        )
    }
}

impl AsLayer<&DubaiPapiConfiguration, HairColor> for HairStyle {
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
            HairStyle::female(),
            &[
                HairStyle::Bob,
                HairStyle::Curly,
                HairStyle::Long,
                HairStyle::PonyTail,
            ]
        );
        assert_eq!(
            HairStyle::male(),
            &[
                HairStyle::Bald,
                HairStyle::Bun,
                HairStyle::IvyLeague,
                HairStyle::Pixie,
                HairStyle::Taper,
            ]
        );
    }
}
