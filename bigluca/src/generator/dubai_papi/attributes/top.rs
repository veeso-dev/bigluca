#[derive(Debug, AllAttributes, Clone, Copy, PartialEq, Eq)]
pub enum Top {
    BlackJacket,
    BlueJacke,
    Shirt,
    TankTop,
    BlackTShirt,
    BlueTShirt,
    GreenTShirt,
    OrangeTShirt,
    RedTShirt,
    WhiteTShirt,
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(
            Top::all(),
            &[
                Top::BlackJacket,
                Top::BlueJacke,
                Top::Shirt,
                Top::TankTop,
                Top::BlackTShirt,
                Top::BlueTShirt,
                Top::GreenTShirt,
                Top::OrangeTShirt,
                Top::RedTShirt,
                Top::WhiteTShirt,
            ]
        )
    }
}
