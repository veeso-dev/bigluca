use crate::nft::{Attribute, IntoAttribute};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Eyes {
    Black,
    Blue,
    Brown,
    Green,
}

impl IntoAttribute for Eyes {
    fn into_attribute(&self) -> Attribute {
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
}
