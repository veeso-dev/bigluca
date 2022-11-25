use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute},
    render::{AsLayer, Layer},
};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum HatColor {
    Black,
    Cyan,
    Green,
    Pink,
    Red,
}

impl AsAttribute for HatColor {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            "Hat Color",
            match self {
                Self::Black => "Black",
                Self::Cyan => "Cyan",
                Self::Green => "Green",
                Self::Pink => "Pink",
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
                Self::Pink => &paths.assets.hat_color.pink,
                Self::Red => &paths.assets.hat_color.red,
            },
            0,
            0,
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
                HatColor::Pink,
                HatColor::Red,
            ]
        )
    }
}
