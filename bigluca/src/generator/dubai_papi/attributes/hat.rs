use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute, FromAttributes},
    render::{AsLayer, Layer},
};

const TRAIT_TYPE: &str = "Hat";

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
            TRAIT_TYPE,
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

impl FromAttributes for Hat {
    fn from_attributes(attributes: &[Attribute]) -> Option<Self> {
        attributes
            .iter()
            .find(|x| x.trait_type == TRAIT_TYPE)
            .and_then(|x| match x.value.as_str() {
                "Black" => Some(Self::Black),
                "Cyan" => Some(Self::Cyan),
                "Green" => Some(Self::Green),
                "Pink" => Some(Self::Pink),
                "Red" => Some(Self::Red),
                _ => None,
            })
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

    #[test]
    fn should_generate_attribute_with_correct_name() {
        assert_eq!(&Hat::Black.as_attribute().trait_type, "Hat");
    }
}
