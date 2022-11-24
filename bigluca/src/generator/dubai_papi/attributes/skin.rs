use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute},
    render::{AsLayer, Layer},
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
    fn path(&self, config: &DubaiPapiConfiguration) -> PathBuf {
        match self {
            Self::Dark => &config.assets.skin.dark,
            Self::Olive => &config.assets.skin.olive,
            Self::White => &config.assets.skin.white,
            Self::Asian => &config.assets.skin.white,
        }
        .to_path_buf()
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

impl AsLayer<&DubaiPapiConfiguration, ()> for Skin {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, _states: ()) -> anyhow::Result<Layer> {
        Layer::from_path(&self.path(paths), 96, 24)
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
