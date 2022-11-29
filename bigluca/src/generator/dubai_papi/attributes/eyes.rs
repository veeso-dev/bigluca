use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute},
    render::{AsLayer, Layer},
};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Eyes {
    Black,
    Blue,
    Brown,
    Green,
}

impl AsAttribute for Eyes {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            "Eyes",
            match self {
                Self::Black => "Black",
                Self::Blue => "Blue",
                Self::Brown => "Brown",
                Self::Green => "Green",
            },
        )
    }
}

impl AsLayer<&DubaiPapiConfiguration, ()> for Eyes {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, _states: ()) -> anyhow::Result<Layer> {
        Layer::from_path(match self {
            Self::Black => &paths.assets.eyes.black,
            Self::Blue => &paths.assets.eyes.blue,
            Self::Brown => &paths.assets.eyes.brown,
            Self::Green => &paths.assets.eyes.green,
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
            Eyes::all(),
            &[Eyes::Black, Eyes::Blue, Eyes::Brown, Eyes::Green,]
        )
    }

    #[test]
    fn should_generate_attribute_with_correct_name() {
        assert_eq!(&Eyes::Green.as_attribute().trait_type, "Eyes");
    }
}
