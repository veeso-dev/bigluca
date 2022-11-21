#[derive(Debug, AllAttributes, Clone, Copy, PartialEq, Eq)]
pub enum CarColor {
    Black,
    Blue,
    Gray,
    Green,
    Orange,
    Pink,
    Red,
    White,
    Yellow,
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(
            CarColor::all(),
            &[
                CarColor::Black,
                CarColor::Blue,
                CarColor::Gray,
                CarColor::Green,
                CarColor::Orange,
                CarColor::Pink,
                CarColor::Red,
                CarColor::White,
                CarColor::Yellow,
            ]
        )
    }
}
