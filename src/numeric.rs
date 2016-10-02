use super::Spec;

use std::fmt::Debug;
use std::cmp::PartialOrd;

pub trait OrderedSpec<T>
    where T: Debug + PartialOrd
{
    fn is_less_than(&mut self, other: &T) -> &mut Self;
    fn is_less_than_or_equal_to(&mut self, other: &T) -> &mut Self;
    fn is_greater_than(&mut self, other: &T) -> &mut Self;
    fn is_greater_than_or_equal_to(&mut self, other: &T) -> &mut Self;
}

impl<'s, T> OrderedSpec<T> for Spec<'s, T>
    where T: Debug + PartialOrd
{
    /// Asserts that the subject is less than the expected value. The subject type must
    /// implement `PartialOrd`.
    ///
    /// ```rust,ignore
    /// assert_that(&1).is_less_than(&2);
    /// ```
    fn is_less_than(&mut self, other: &T) -> &mut Self {
        let subject = self.subject;

        if subject >= other {
            self.with_expected(format!("value less than <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }

        self
    }

    /// Asserts that the subject is less than or equal to the expected value. The subject type
    /// must implement `PartialOrd`.
    ///
    /// ```rust,ignore
    /// assert_that(&2).is_less_than_or_equal_to(&2);
    /// ```
    fn is_less_than_or_equal_to(&mut self, other: &T) -> &mut Self {
        let subject = self.subject;

        if subject > other {
            self.with_expected(format!("value less than or equal to <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }

        self
    }

    /// Asserts that the subject is greater than the expected value. The subject type must
    /// implement `PartialOrd`.
    ///
    /// ```rust,ignore
    /// assert_that(&2).is_greater_than(&1);
    /// ```
    fn is_greater_than(&mut self, other: &T) -> &mut Self {
        let subject = self.subject;

        if subject <= other {
            self.with_expected(format!("value greater than <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }

        self
    }

    /// Asserts that the subject is greater than or equal to the expected value. The subject
    /// type must implement `PartialOrd`.
    ///
    /// ```rust,ignore
    /// assert_that(&2).is_greater_than_or_equal_to(&1);
    /// ```
    fn is_greater_than_or_equal_to(&mut self, other: &T) -> &mut Self {
        let subject = self.subject;

        if subject < other {
            self.with_expected(format!("value greater than or equal to <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }

        self
    }
}
