use super::{AssertionFailure, Spec};

use std::borrow::Borrow;
use std::cmp::PartialEq;
use std::fmt::Debug;

pub trait OptionAssertions<'r, T>
where
    T: Debug,
{
    fn is_some(&mut self) -> Spec<'r, T>;
    fn is_none(&mut self);
}

pub trait ContainingOptionAssertions<T>
where
    T: Debug + PartialEq,
{
    fn contains_value<E: Borrow<T>>(&mut self, expected_value: E);
}

impl<'s, T> ContainingOptionAssertions<T> for Spec<'s, Option<T>>
where
    T: Debug + PartialEq,
{
    /// Asserts that the subject is a `Some` containing the expected value. The subject type must
    /// be an `Option`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&Some(1)).contains_value(&1);
    /// ```
    fn contains_value<E: Borrow<T>>(&mut self, expected_value: E) {
        let borrowed_expected_value = expected_value.borrow();

        match *self.subject {
            Some(ref val) => {
                if !val.eq(borrowed_expected_value) {
                    AssertionFailure::from_spec(self)
                        .with_expected(format!("option to contain <{:?}>", borrowed_expected_value))
                        .with_actual(format!("<{:?}>", val))
                        .fail();
                }
            }
            None => {
                AssertionFailure::from_spec(self)
                    .with_expected(format!("option<{:?}>", borrowed_expected_value))
                    .with_actual("option[none]".to_string())
                                     .fail();
            }
        };
    }
}

impl<'s, T> OptionAssertions<'s, T> for Spec<'s, Option<T>>
where
    T: Debug,
{
    /// Asserts that the subject is `Some`. The subject type must be an `Option`.
    ///
    /// This will return a new `Spec` containing the unwrapped value if it is `Some`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&Some(1)).is_some();
    /// ```
    fn is_some(&mut self) -> Spec<'s, T> {
        match *self.subject {
            Some(ref val) => Spec {
                subject: val,
                subject_name: self.subject_name,
                location: self.location.clone(),
                description: self.description,
            },
            None => {
                AssertionFailure::from_spec(self)
                    .with_expected("option[some]".to_string())
                    .with_actual("option[none]".to_string())
                    .fail();

                unreachable!();
            }
        }
    }

    /// Asserts that the subject is `None`. The value type must be an `Option`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&Option::None::<String>).is_none();
    /// ```
    fn is_none(&mut self) {
        match *self.subject {
            None => (),
            Some(ref val) => {
                AssertionFailure::from_spec(self)
                    .with_expected("option[none]".to_string())
                    .with_actual(format!("option<{:?}>", val))
                    .fail();
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;

    #[test]
    fn should_not_panic_if_option_is_expected_to_contain_value_and_does() {
        let option = Some("Hello");
        assert_that(&option).is_some();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: option[some]\n\t but was: option[none]")]
    fn should_panic_if_option_is_expected_to_contain_value_and_does_not() {
        let option: Option<&str> = None;
        assert_that(&option).is_some();
    }

    #[test]
    fn should_be_able_to_unwrap_option_if_some() {
        let option = Some("Hello");
        assert_that(&option).is_some().is_equal_to(&"Hello");
    }

    #[test]
    fn contains_value_should_allow_multiple_borrow_types() {
        let option = Some("Hello");
        assert_that(&option).contains_value("Hello");
        assert_that(&option).contains_value(&mut "Hello");
        assert_that(&option).contains_value(&"Hello");
    }

    #[test]
    fn should_not_panic_if_option_contains_expected_value() {
        let option = Some("Hello");
        assert_that(&option).contains_value(&"Hello");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: option to contain <\"Hi\">\n\t but was: <\"Hello\">")]
    fn should_panic_if_option_does_not_contain_expected_value() {
        let option = Some("Hello");
        assert_that(&option).contains_value(&"Hi");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: option<\"Hello\">\n\t but was: option[none]")]
    fn should_panic_if_option_is_none_but_expected_value() {
        let option: Option<&str> = None;
        assert_that(&option).contains_value(&"Hello");
    }

    #[test]
    fn should_not_panic_if_option_is_empty() {
        let option: Option<&str> = None;
        assert_that(&option).is_none();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: option[none]\n\t but was: option<\"Hello\"")]
    fn should_panic_if_option_is_not_empty_but_was_expected_as_empty() {
        let option = Some("Hello");
        assert_that(&option).is_none();
    }
}
