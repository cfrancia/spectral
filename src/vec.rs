use super::Spec;

pub trait VecSpec {
    fn has_length(&mut self, expected: usize) -> &mut Self;
}

impl<'s, T> VecSpec for Spec<'s, Vec<T>> {
    /// Asserts that the length of the subject vector is equal to the provided length. The subject
    /// type must be of `Vec`.
    ///
    /// ```rust,ignore
    /// assert_that(&vec![1, 2, 3, 4]).has_length(4);
    /// ```
    fn has_length(&mut self, expected: usize) -> &mut Self {
        let length = self.subject.len();
        if length != expected {
            self.with_expected(format!("vec to have length <{}>", expected))
                .with_actual(format!("<{}>", length))
                .fail();
        }

        self
    }
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;

    #[test]
    fn should_not_panic_if_vec_length_matches_expected() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).has_length(3);
    }

    #[test]
    #[should_panic(expected = "expected vec to have length <1> but was <3>")]
    fn should_panic_if_vec_length_does_not_match_expected() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).has_length(1);
    }

}
