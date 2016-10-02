use super::Spec;

use std::cmp::PartialEq;
use std::fmt::Debug;

pub trait ComparingIterSpec<'s, T: 's>
    where T: Debug + PartialEq
{
    fn contains(&mut self, expected_value: &T) -> &mut Self;
}

pub trait MatchingIterSpec<'s, T: 's>
    where T: Debug
{
    fn matching_contains<F>(&mut self, matcher: F) -> &mut Self where F: Fn(&'s T) -> bool;
}

impl<'s, T: 's, I> ComparingIterSpec<'s, T> for Spec<'s, I>
    where T: Debug + PartialEq,
          &'s I: IntoIterator<Item = &'s T>
{
    /// Asserts that the subject contains the provided value. The subject must implement
    /// `IntoIterator`, and the contained type must implement `PartialEq` and `Debug`.
    ///
    /// ```rust,ignore
    /// let test_vec = vec![1,2,3];
    /// assert_that(&test_vec).contains(&2);
    /// ```
    fn contains(&mut self, expected_value: &T) -> &mut Self {
        let mut actual = Vec::new();
        for x in self.subject {
            if expected_value.eq(x) {
                return self;
            } else {
                actual.push(x);
            }
        }
        self.with_expected(format!("iterator to contain <{:?}>", expected_value))
            .with_actual(format!("<{:?}>", actual))
            .fail();

        unreachable!();
    }
}

impl<'s, T: 's, I> MatchingIterSpec<'s, T> for Spec<'s, I>
    where T: Debug,
          &'s I: IntoIterator<Item = &'s T>
{
    /// Asserts that the subject contains a matching item by using the provided function.
    /// The subject must implement `IntoIterator`, and the contained type must implement `Debug`.
    ///
    /// ```rust,ignore
    /// let mut test_into_iter = LinkedList::new();
    /// test_into_iter.push_back(TestEnum::Bad);
    /// test_into_iter.push_back(TestEnum::Good);
    /// test_into_iter.push_back(TestEnum::Bad);
    ///
    /// assert_that(&test_into_iter).matching_contains(|val| {
    ///     match val {
    ///         &TestEnum::Good => true,
    ///         _ => false
    ///     }
    /// });
    /// ```
    fn matching_contains<F>(&mut self, matcher: F) -> &mut Self
        where F: Fn(&'s T) -> bool
    {
        let mut actual = Vec::new();
        for x in self.subject {
            if matcher(x) {
                return self;
            } else {
                actual.push(x);
            }
        }
        self.fail_with_message(format!("expectation failed for iterator with values <{:?}>",
                                       actual));

        unreachable!();
    }
}
