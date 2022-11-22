use crate::nft::{Attribute, IntoAttribute};

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
            "Top",
            match self {
                Self::Black => "Black",
                Self::Cyan => "Cyan",
                Self::Green => "Green",
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
