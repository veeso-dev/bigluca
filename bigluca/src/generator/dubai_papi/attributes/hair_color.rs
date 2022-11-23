use crate::nft::{Attribute, IntoAttribute};

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

impl IntoAttribute for HairColor {
    fn into_attribute(&self) -> Attribute {
        Attribute::new(
            "Hair Color",
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
}
