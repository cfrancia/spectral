use super::Spec;

pub trait StrSpec {
    fn starts_with(&mut self, expected: &str) -> &mut Self;
    fn ends_with(&mut self, expected: &str) -> &mut Self;
    fn contains(&mut self, expected: &str) -> &mut Self;
}

impl<'s> StrSpec for Spec<'s, &'s str> {
    /// Asserts that the subject `&str` starts with the provided `&str`.
    ///
    /// ```rust,ignore
    /// assert_that(&"Hello").starts_with(&"H");
    /// ```
    fn starts_with(&mut self, expected: &str) -> &mut Self {
        let subject = self.subject;

        if !subject.starts_with(expected) {
            self.with_expected(format!("string starting with <{:?}>", expected))
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
            self.with_expected(format!("string ending with <{:?}>", expected))
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
            self.with_expected(format!("string containing <{:?}>", expected))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }

        self
    }
}
