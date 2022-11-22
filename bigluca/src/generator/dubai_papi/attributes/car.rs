#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum Car {
    Baguette,
    Lambo,
    Nikola,
    RollsRoyal,
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
