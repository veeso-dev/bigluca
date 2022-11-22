use crate::nft::{Attribute, IntoAttribute};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Top {
    BlackJacket,
    BlueJacket,
    Shirt,
    TankTop,
    BlackTShirt,
    BlueTShirt,
    GreenTShirt,
    OrangeTShirt,
    RedTShirt,
    WhiteTShirt,
}

impl IntoAttribute for Top {
    fn into_attribute(&self) -> Attribute {
        Attribute::new(
            "Top",
            match self {
                Self::BlackJacket => "Black Jacket",
                Self::BlueJacket => "Blue Jacket",
                Self::Shirt => "Shirt",
                Self::TankTop => "Tank Top",
                Self::BlackTShirt => "Black T-Shirt",
                Self::BlueTShirt => "Blue T-Shirt",
                Self::GreenTShirt => "Green T-Shirt",
                Self::OrangeTShirt => "Orange T-Shirt",
                Self::RedTShirt => "Red T-Shirt",
                Self::WhiteTShirt => "White T-Shirt",
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
            Top::all(),
            &[
                Top::BlackJacket,
                Top::BlueJacket,
                Top::Shirt,
                Top::TankTop,
                Top::BlackTShirt,
                Top::BlueTShirt,
                Top::GreenTShirt,
                Top::OrangeTShirt,
                Top::RedTShirt,
                Top::WhiteTShirt,
            ]
        )
    }
}
