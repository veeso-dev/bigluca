#[derive(Debug, AllAttributes, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
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
