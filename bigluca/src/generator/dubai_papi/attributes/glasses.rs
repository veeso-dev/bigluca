use crate::nft::{Attribute, IntoAttribute};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Glasses {
    Eyeglasses,
    Sunglasses,
}

impl IntoAttribute for Glasses {
    fn into_attribute(&self) -> Attribute {
        Attribute::new(
            "Glasses",
            match self {
                Self::Eyeglasses => "Eyeglasses",
                Self::Sunglasses => "Sunglasses",
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
        assert_eq!(Glasses::all(), &[Glasses::Eyeglasses, Glasses::Sunglasses])
    }
}
