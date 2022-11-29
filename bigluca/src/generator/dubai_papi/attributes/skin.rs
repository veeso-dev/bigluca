use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute},
    render::{AsLayer, Layer},
};

use std::path::PathBuf;

use super::Gender;

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Skin {
    Dark,
    Olive,
    White,
    Asian,
}

impl Skin {
    fn path(&self, config: &DubaiPapiConfiguration, gender: Gender) -> PathBuf {
        let mut p = match gender {
            Gender::Male => config.assets.skin.male.clone(),
            Gender::Female => config.assets.skin.female.clone(),
        };
        p.extend(match self {
            Self::Dark => &config.assets.skin.dark,
            Self::Olive => &config.assets.skin.olive,
            Self::White => &config.assets.skin.white,
            Self::Asian => &config.assets.skin.asian,
        });

        p
    }
}

impl AsAttribute for Skin {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            "Skin",
            match self {
                Self::Dark => "Dark",
                Self::Olive => "Olive",
                Self::White => "White",
                Self::Asian => "Asian",
            },
        )
    }
}

impl AsLayer<&DubaiPapiConfiguration, Gender> for Skin {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, states: Gender) -> anyhow::Result<Layer> {
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
            Skin::all(),
            &[Skin::Dark, Skin::Olive, Skin::White, Skin::Asian,]
        )
    }

    #[test]
    fn should_generate_attribute_with_correct_name() {
        assert_eq!(&Skin::Asian.as_attribute().trait_type, "Skin");
    }
}
