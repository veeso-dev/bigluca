use crate::nft::{AsAttribute, Attribute, FromAttributes};

const TRAIT_TYPE: &str = "Hair Color";

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum HairColor {
    Black,
    Brown,
    Blonde,
    Blue,
    Green,
    Pink,
    Red,
}

impl HairColor {
    /// Return all variants without blonde
    pub fn no_blonde() -> &'static [Self] {
        &[
            Self::Black,
            Self::Brown,
            Self::Blue,
            Self::Green,
            Self::Pink,
            Self::Red,
        ]
    }
}

impl AsAttribute for HairColor {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            TRAIT_TYPE,
            match self {
                Self::Black => "Black",
                Self::Brown => "Brown",
                Self::Blonde => "Blonde",
                Self::Blue => "Blue",
                Self::Green => "Green",
                Self::Pink => "Pink",
                Self::Red => "Red",
            },
        )
    }
}

impl FromAttributes for HairColor {
    fn from_attributes(attributes: &[Attribute]) -> Option<Self> {
        attributes
            .iter()
            .find(|x| x.trait_type == TRAIT_TYPE)
            .map(|x| match x.value.as_str() {
                "Black" => Some(Self::Black),
                "Brown" => Some(Self::Brown),
                "Blonde" => Some(Self::Blonde),
                "Blue" => Some(Self::Blue),
                "Green" => Some(Self::Green),
                "Pink" => Some(Self::Pink),
                "Red" => Some(Self::Red),
                _ => None,
            })
            .flatten()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(
            HairColor::all(),
            &[
                HairColor::Black,
                HairColor::Brown,
                HairColor::Blonde,
                HairColor::Blue,
                HairColor::Green,
                HairColor::Pink,
                HairColor::Red,
            ]
        )
    }

    #[test]
    fn should_generate_attribute_with_correct_name() {
        assert_eq!(&HairColor::Red.as_attribute().trait_type, "Hair Color");
    }
}
