//! # Collisions

/// Given a function which can create collisions with values, try to call `getter` for
/// up to `attempts` until it returns some. If the function didn't return any value by
/// n `attempts`, then return error
pub fn try_for<F, T>(mut getter: F, attempts: usize) -> anyhow::Result<T>
where
    F: FnMut() -> Option<T>,
{
    for _ in 0..attempts {
        if let Some(val) = getter() {
            return Ok(val);
        }
    }
    anyhow::bail!("could not get value after {} attempts", attempts)
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_get_value_after_n_attempts() {
        let mut counter = 0;
        assert_eq!(
            try_for(
                || {
                    counter += 1;
                    if counter == 32 {
                        Some(1)
                    } else {
                        None
                    }
                },
                60
            )
            .unwrap(),
            1
        );
    }

    #[test]
    fn should_not_get_value_after_n_attempts() {
        assert!(try_for(|| if 0 == 1 { Some(1) } else { None }, 100).is_err());
    }
}
