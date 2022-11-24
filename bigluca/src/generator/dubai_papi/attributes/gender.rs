use crate::nft::{AsAttribute, Attribute};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

impl AsAttribute for Gender {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            "Gender",
            match self {
                Self::Female => "Female",
                Self::Male => "Male",
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
        assert_eq!(Gender::all(), &[Gender::Male, Gender::Female])
    }
}
