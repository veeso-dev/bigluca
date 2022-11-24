//! # Random
//!
//! Random utilities

#[cfg(test)]
use rand::distributions::Alphanumeric;
use rand::{rngs::ThreadRng, thread_rng, Rng};

pub struct Random {
    rng: ThreadRng,
}

impl Default for Random {
    fn default() -> Self {
        Self { rng: thread_rng() }
    }
}

impl Random {
    /// Choose a random element from `choices`
    pub fn choice<'a, T>(&mut self, choices: &'a [T]) -> &'a T {
        &choices[self.rng.gen_range(0..choices.len())]
    }

    /// Choose a random element from `choices` with a probability of `some_probability`%.
    /// Otherwise will return None
    pub fn choice_or_none<'a, T>(
        &mut self,
        choices: &'a [T],
        some_probability: u8,
    ) -> Option<&'a T> {
        if self.happens(some_probability) {
            Some(self.choice(choices))
        } else {
            None
        }
    }

    /// Given a percentage, returns whether the event should happen
    /// Panics if `probability` is out of range 1-100
    pub fn happens(&mut self, probability: u8) -> bool {
        assert!(probability <= 100);
        self.rng.gen_range(0..100) < probability
    }

    #[cfg(test)]
    /// Generate a random alphanumeric string with provided length
    pub fn random_alphanumeric_with_len(&mut self, len: usize) -> String {
        std::iter::repeat(())
            .map(|()| self.rng.sample(Alphanumeric))
            .map(char::from)
            .take(len)
            .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_tell_whether_event_happens() {
        assert!(Random::default().happens(100));
        assert_eq!(Random::default().happens(0), false);
    }

    #[test]
    fn should_make_choice() {
        assert!(&[1, 2, 3].contains(Random::default().choice(&[1, 2, 3])));
    }

    #[test]
    fn should_make_choice_or_return_none() {
        assert!(&[1, 2, 3].contains(Random::default().choice_or_none(&[1, 2, 3], 100).unwrap()));
        assert!(Random::default().choice_or_none(&[1, 2, 3], 0).is_none());
    }

    #[test]
    fn should_generate_random_alphanumeric_with_len() {
        assert_eq!(
            Random::default().random_alphanumeric_with_len(256).len(),
            256
        );
    }
}
