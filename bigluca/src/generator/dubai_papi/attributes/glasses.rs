use crate::{
    config::DubaiPapiConfiguration,
    nft::{Attribute, IntoAttribute},
    render::{AsLayer, Layer},
};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Glasses {
    Eyeglasses,
    Sunglasses,
}

impl IntoAttribute for Glasses {
    fn into_attribute(&self) -> Attribute {
        Attribute::new(
            "Glasses",
            match self {
                Self::Eyeglasses => "Eyeglasses",
                Self::Sunglasses => "Sunglasses",
            },
        )
    }
}

impl AsLayer<&DubaiPapiConfiguration, ()> for Glasses {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, _states: ()) -> anyhow::Result<Layer> {
        Layer::from_path(
            match self {
                Self::Eyeglasses => &paths.assets.glasses.eyeglasses,
                Self::Sunglasses => &paths.assets.glasses.sunglasses,
            },
            128,
            32,
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(Glasses::all(), &[Glasses::Eyeglasses, Glasses::Sunglasses])
    }
}
