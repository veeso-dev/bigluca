use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute, FromAttributes},
    render::{AsLayer, Layer},
};

const TRAIT_TYPE: &str = "Eyes";

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
            TRAIT_TYPE,
            match self {
                Self::Black => "Black",
                Self::Blue => "Blue",
                Self::Brown => "Brown",
                Self::Green => "Green",
            },
        )
    }
}

impl FromAttributes for Eyes {
    fn from_attributes(attributes: &[Attribute]) -> Option<Self> {
        attributes
            .iter()
            .find(|x| x.trait_type == TRAIT_TYPE)
            .map(|x| match x.value.as_str() {
                "Black" => Some(Self::Black),
                "Blue" => Some(Self::Blue),
                "Brown" => Some(Self::Brown),
                "Green" => Some(Self::Green),
                _ => None,
            })
            .flatten()
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
