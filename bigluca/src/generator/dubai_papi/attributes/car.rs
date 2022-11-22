use crate::nft::{Attribute, IntoAttribute};

#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Car {
    Baguette,
    Lambo,
    Nikola,
    RollsRoyal,
}

impl IntoAttribute for Car {
    fn into_attribute(&self) -> Attribute {
        Attribute::new(
            "Car",
            match self {
                Self::Baguette => "Baguette",
                Self::Lambo => "Lambo",
                Self::Nikola => "Nikola",
                Self::RollsRoyal => "RollsRoyal",
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
            Car::all(),
            &[Car::Baguette, Car::Lambo, Car::Nikola, Car::RollsRoyal,]
        )
    }
}
