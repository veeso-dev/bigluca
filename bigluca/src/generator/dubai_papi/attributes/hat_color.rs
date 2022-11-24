use crate::{
    config::DubaiPapiConfiguration,
    nft::{Attribute, IntoAttribute},
    render::{AsLayer, Layer},
};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum HatColor {
    Black,
    Cyan,
    Green,
    Red,
}

impl IntoAttribute for HatColor {
    fn into_attribute(&self) -> Attribute {
        Attribute::new(
            "Hat Color",
            match self {
                Self::Black => "Black",
                Self::Cyan => "Cyan",
                Self::Green => "Green",
                Self::Red => "Red",
            },
        )
    }
}

impl AsLayer<&DubaiPapiConfiguration, ()> for HatColor {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, _states: ()) -> anyhow::Result<Layer> {
        Layer::from_path(
            match self {
                Self::Black => &paths.assets.hat_color.black,
                Self::Cyan => &paths.assets.hat_color.cyan,
                Self::Green => &paths.assets.hat_color.green,
                Self::Red => &paths.assets.hat_color.red,
            },
            140,
            36,
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
            HatColor::all(),
            &[
                HatColor::Black,
                HatColor::Cyan,
                HatColor::Green,
                HatColor::Red,
            ]
        )
    }
}
