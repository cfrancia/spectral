use super::{AssertionFailure, Spec};

pub trait StrAssertions {
    fn starts_with(&mut self, expected: &str) -> &mut Self;
    fn ends_with(&mut self, expected: &str) -> &mut Self;
    fn contains(&mut self, expected: &str) -> &mut Self;
}

impl<'s> StrAssertions for Spec<'s, &'s str> {
    /// Asserts that the subject `&str` starts with the provided `&str`.
    ///
    /// ```rust,ignore
    /// assert_that(&"Hello").starts_with(&"H");
    /// ```
    fn starts_with(&mut self, expected: &str) -> &mut Self {
        let subject = self.subject;

        if !subject.starts_with(expected) {
            AssertionFailure::from_spec(self)
                .with_expected(format!("string starting with <{:?}>", expected))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }

        self
    }

    /// Asserts that the subject `&str` ends with the provided `&str`.
    ///
    /// ```rust,ignore
    /// assert_that(&"Hello").ends_with(&"o");
    /// ```
    fn ends_with(&mut self, expected: &str) -> &mut Self {
        let subject = self.subject;

        if !subject.ends_with(expected) {
            AssertionFailure::from_spec(self)
                .with_expected(format!("string ending with <{:?}>", expected))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }

        self
    }

    /// Asserts that the subject `&str` contains the provided `&str`.
    ///
    /// ```rust,ignore
    /// assert_that(&"Hello").contains(&"e");
    /// ```
    fn contains(&mut self, expected: &str) -> &mut Self {
        let subject = self.subject;

        if !subject.contains(expected) {
            AssertionFailure::from_spec(self)
                .with_expected(format!("string containing <{:?}>", expected))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }

        self
    }
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;

    #[test]
    fn should_not_panic_if_str_starts_with_value() {
        let value = "Hello";
        assert_that(&value).starts_with(&"H");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string starting with <\"A\">\
                   \n\t but was: <\"Hello\">")]
    fn should_panic_if_str_does_not_start_with_value() {
        let value = "Hello";
        assert_that(&value).starts_with(&"A");
    }

    #[test]
    fn should_not_panic_if_str_ends_with_value() {
        let value = "Hello";
        assert_that(&value).ends_with(&"o");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string ending with <\"A\">\n\t but was: <\"Hello\">")]
    fn should_panic_if_str_does_not_end_with_value() {
        let value = "Hello";
        assert_that(&value).ends_with(&"A");
    }

    #[test]
    fn should_not_panic_if_str_contains_value() {
        let value = "Hello";
        assert_that(&value).contains(&"l");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string containing <\"A\">\n\t but was: <\"Hello\">")]
    fn should_panic_if_str_does_not_contain_value() {
        let value = "Hello";
        assert_that(&value).contains(&"A");
    }

}
