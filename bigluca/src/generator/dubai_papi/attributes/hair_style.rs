use crate::nft::{Attribute, IntoAttribute};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum HairStyle {
    Bald,
    BobCut,
}

impl IntoAttribute for HairStyle {
    fn into_attribute(&self) -> Attribute {
        Attribute::new(
            "Hair Style",
            match self {
                Self::Bald => "Bald",
                Self::BobCut => "Bob Cut",
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
        assert_eq!(HairStyle::all(), &[HairStyle::Bald, HairStyle::BobCut])
    }
}
