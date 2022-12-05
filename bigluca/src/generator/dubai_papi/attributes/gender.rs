use crate::nft::{AsAttribute, Attribute, FromAttributes};

const TRAIT_TYPE: &str = "Gender";

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

impl AsAttribute for Gender {
    fn as_attribute(&self) -> Attribute {
        Attribute::new(
            TRAIT_TYPE,
            match self {
                Self::Female => "Female",
                Self::Male => "Male",
            },
        )
    }
}

impl FromAttributes for Gender {
    fn from_attributes(attributes: &[Attribute]) -> Option<Self> {
        attributes
            .iter()
            .find(|x| x.trait_type == TRAIT_TYPE)
            .and_then(|x| match x.value.as_str() {
                "Female" => Some(Self::Female),
                "Male" => Some(Self::Male),
                _ => None,
            })
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

    #[test]
    fn should_generate_attribute_with_correct_name() {
        assert_eq!(&Gender::Male.as_attribute().trait_type, "Gender");
    }
}
