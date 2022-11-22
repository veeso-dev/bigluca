use crate::nft::{Attribute, IntoAttribute};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum EarPods {
    Black,
    White,
}

impl IntoAttribute for EarPods {
    fn into_attribute(&self) -> Attribute {
        Attribute::new(
            "Ear Pods",
            match self {
                Self::Black => "Black",
                Self::White => "White",
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
        assert_eq!(EarPods::all(), &[EarPods::Black, EarPods::White])
    }
}
