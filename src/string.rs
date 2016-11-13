use super::{AssertionFailure, Spec};

use std::borrow::Borrow;

pub trait StrAssertions {
    fn starts_with<'r, E: Borrow<&'r str>>(&mut self, expected: E);
    fn ends_with<'r, E: Borrow<&'r str>>(&mut self, expected: E);
    fn contains<'r, E: Borrow<&'r str>>(&mut self, expected: E);
}

impl<'s> StrAssertions for Spec<'s, &'s str> {
    /// Asserts that the subject `&str` starts with the provided `&str`.
    ///
    /// ```rust,ignore
    /// assert_that(&"Hello").starts_with(&"H");
    /// ```
    fn starts_with<'r, E: Borrow<&'r str>>(&mut self, expected: E) {
        let subject = self.subject;
        let borrowed_expected = expected.borrow();

        if !subject.starts_with(borrowed_expected) {
            AssertionFailure::from_spec(self)
                .with_expected(format!("string starting with <{:?}>", borrowed_expected))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }

    /// Asserts that the subject `&str` ends with the provided `&str`.
    ///
    /// ```rust,ignore
    /// assert_that(&"Hello").ends_with(&"o");
    /// ```
    fn ends_with<'r, E: Borrow<&'r str>>(&mut self, expected: E) {
        let subject = self.subject;
        let borrowed_expected = expected.borrow();

        if !subject.ends_with(borrowed_expected) {
            AssertionFailure::from_spec(self)
                .with_expected(format!("string ending with <{:?}>", borrowed_expected))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }

    /// Asserts that the subject `&str` contains the provided `&str`.
    ///
    /// ```rust,ignore
    /// assert_that(&"Hello").contains(&"e");
    /// ```
    fn contains<'r, E: Borrow<&'r str>>(&mut self, expected: E) {
        let subject = self.subject;
        let borrowed_expected = expected.borrow();

        if !subject.contains(borrowed_expected) {
            AssertionFailure::from_spec(self)
                .with_expected(format!("string containing <{:?}>", borrowed_expected))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;

    #[test]
    fn should_allow_multiple_borrow_forms() {
        let value = "Hello";
        assert_that(&value).starts_with("H");
        assert_that(&value).starts_with(&mut "H");
        assert_that(&value).starts_with(&"H");

        assert_that(&value).ends_with("o");
        assert_that(&value).ends_with(&mut "o");
        assert_that(&value).ends_with(&"o");

        assert_that(&value).contains("l");
        assert_that(&value).contains(&mut "l");
        assert_that(&value).contains(&"l");
    }

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
