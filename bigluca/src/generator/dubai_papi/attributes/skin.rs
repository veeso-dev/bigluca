use super::Gender;
use crate::{
    config::DubaiPapiConfiguration,
    nft::{Attribute, IntoAttribute},
};
use std::path::PathBuf;

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
            Self::Asian => &config.assets.skin.white,
        });

        p
    }
}

impl IntoAttribute for Skin {
    fn into_attribute(&self) -> Attribute {
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
}
