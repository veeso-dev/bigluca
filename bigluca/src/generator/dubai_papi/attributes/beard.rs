use crate::nft::{Attribute, IntoAttribute};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Beard {}

impl IntoAttribute for Beard {
    fn into_attribute(&self) -> Attribute {
        Attribute::new("Car Color", "niente")
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(Beard::all(), &[])
    }
}