use crate::nft::{Attribute, IntoAttribute};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum HairStyle {
    Bald,
}

impl IntoAttribute for HairStyle {
    fn into_attribute(&self) -> Attribute {
        Attribute::new(
            "Top",
            match self {
                Self::Bald => "Bald",
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
        assert_eq!(HairStyle::all(), &[HairStyle::Bald])
    }
}
