use crate::nft::{AsAttribute, Attribute};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum CarColor {
    Black,
    Blue,
    Gray,
    Green,
    Orange,
    Pink,
    Red,
    White,
    Yellow,
}

impl AsAttribute for CarColor {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            "Car Color",
            match self {
                Self::Black => "Black",
                Self::Blue => "Blue",
                Self::Gray => "Gray",
                Self::Green => "Green",
                Self::Orange => "Orange",
                Self::Pink => "Pink",
                Self::Red => "Red",
                Self::White => "White",
                Self::Yellow => "Yellow",
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
            CarColor::all(),
            &[
                CarColor::Black,
                CarColor::Blue,
                CarColor::Gray,
                CarColor::Green,
                CarColor::Orange,
                CarColor::Pink,
                CarColor::Red,
                CarColor::White,
                CarColor::Yellow,
            ]
        )
    }
}
