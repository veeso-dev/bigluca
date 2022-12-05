use crate::{
    config::DubaiPapiConfiguration,
    nft::{AsAttribute, Attribute, FromAttributes},
    render::{AsLayer, Layer},
};

const TRAIT_TYPE: &str = "Glasses";

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Glasses {
    Eyeglasses,
    Sunglasses,
}

impl AsAttribute for Glasses {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            TRAIT_TYPE,
            match self {
                Self::Eyeglasses => "Eyeglasses",
                Self::Sunglasses => "Sunglasses",
            },
        )
    }
}

impl FromAttributes for Glasses {
    fn from_attributes(attributes: &[Attribute]) -> Option<Self> {
        attributes
            .iter()
            .find(|x| x.trait_type == TRAIT_TYPE)
            .map(|x| match x.value.as_str() {
                "Eyeglasses" => Some(Self::Eyeglasses),
                "Sunglasses" => Some(Self::Sunglasses),
                _ => None,
            })
            .flatten()
    }
}

impl AsLayer<&DubaiPapiConfiguration, ()> for Glasses {
    fn as_layer(&self, paths: &DubaiPapiConfiguration, _states: ()) -> anyhow::Result<Layer> {
        Layer::from_path(match self {
            Self::Eyeglasses => &paths.assets.glasses.eyeglasses,
            Self::Sunglasses => &paths.assets.glasses.sunglasses,
        })
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

    #[test]
    fn should_generate_attribute_with_correct_name() {
        assert_eq!(&Glasses::Sunglasses.as_attribute().trait_type, "Glasses");
    }
}
