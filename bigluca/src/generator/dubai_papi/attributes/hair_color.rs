#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum HairColor {
    Black,
    Brown,
    Blonde,
    Blue,
    Green,
    Pink,
    Red,
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(
            HairColor::all(),
            &[
                HairColor::Black,
                HairColor::Brown,
                HairColor::Blonde,
                HairColor::Blue,
                HairColor::Green,
                HairColor::Pink,
                HairColor::Red,
            ]
        )
    }
}
