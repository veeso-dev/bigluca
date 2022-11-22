use crate::nft::{Attribute, IntoAttribute};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Skin {
    Dark,
    Olive,
    White,
    Asian,
}

impl IntoAttribute for Skin {
    fn into_attribute(&self) -> Attribute {
        Attribute::new(
            "Top",
            match self {
                Self::Dark => "Dark",
                Self::Olive => "Olive",
                Self::White => "White",
                Self::Asian => "Asian",
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
            Skin::all(),
            &[Skin::Dark, Skin::Olive, Skin::White, Skin::Asian,]
        )
    }
}
