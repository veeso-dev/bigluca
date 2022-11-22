#[derive(Debug, AllVariants, Clone, Copy, PartialEq, Eq)]
pub enum EarPods {
    Black,
    White,
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_all_attributes() {
        assert_eq!(EarPods::all(), &[EarPods::Black, EarPods::White])
    }
}
