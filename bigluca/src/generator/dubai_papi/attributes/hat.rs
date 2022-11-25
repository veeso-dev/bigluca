use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute},
    render::{AsLayer, Layer},
};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Hat {
    Black,
    Cyan,
    Green,
    Pink,
    Red,
}

impl AsAttribute for Hat {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            "Hat",
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

impl AsLayer<&DubaiPapiConfiguration, ()> for Hat {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, _states: ()) -> anyhow::Result<Layer> {
        Layer::from_path(match self {
            Self::Black => &paths.assets.hat.black,
            Self::Cyan => &paths.assets.hat.cyan,
            Self::Green => &paths.assets.hat.green,
            Self::Pink => &paths.assets.hat.pink,
            Self::Red => &paths.assets.hat.red,
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(
            Hat::all(),
            &[Hat::Black, Hat::Cyan, Hat::Green, Hat::Pink, Hat::Red,]
        )
    }
}
