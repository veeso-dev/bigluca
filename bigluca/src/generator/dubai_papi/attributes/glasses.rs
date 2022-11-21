#[derive(Debug, AllAttributes, Clone, Copy, PartialEq, Eq)]
pub enum Glasses {
    Eyeglasses,
    Sunglasses,
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(Glasses::all(), &[Glasses::Eyeglasses, Glasses::Sunglasses])
    }
}
