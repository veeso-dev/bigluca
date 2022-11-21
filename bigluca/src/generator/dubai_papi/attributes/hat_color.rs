#[derive(Debug, AllAttributes, Clone, Copy, PartialEq, Eq)]
pub enum HatColor {
    Black,
    Cyan,
    Green,
    Red,
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(
            HatColor::all(),
            &[
                HatColor::Black,
                HatColor::Cyan,
                HatColor::Green,
                HatColor::Red,
            ]
        )
    }
}
