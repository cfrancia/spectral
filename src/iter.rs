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

pub trait MappingComparingIterSpec<'s, T: 's>
    where T: Debug
{
    fn mapped_contains<F, M: 's>(&mut self, mapping_function: F, expected_value: &M) -> &mut Self
        where M: Debug + PartialEq,
              F: Fn(&'s T) -> M;
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

        panic_unmatched(self, expected_value, actual);
        unreachable!();
    }
}

impl<'s, T: 's, I> MappingComparingIterSpec<'s, T> for Spec<'s, I>
    where T: Debug,
          &'s I: IntoIterator<Item = &'s T>
{
    /// Maps the values of the subject before asserting that the mapped subject contains the
    /// provided value. The subject must implement IntoIterator, and the type of the mapped
    /// value must implement `PartialEq`.
    ///
    /// NOTE: The panic message will refer to the mapped values rather than the values present in
    /// the original subject.
    ///
    /// ```rust,ignore
    /// #[derive(PartialEq, Debug)]
    /// struct Simple {
    ///     pub val: usize,
    /// }
    ///
    /// ...
    ///
    /// assert_that(&vec![Simple { val: 1 }, Simple { val: 2 } ]).mapped_contains(|x| &x.val, &2);
    /// ```
    fn mapped_contains<F, M: 's>(&mut self, mapping_function: F, expected_value: &M) -> &mut Self
        where M: Debug + PartialEq,
              F: Fn(&'s T) -> M
    {
        let subject = self.subject;

        let mapped_vec: Vec<M> = subject.into_iter().map(mapping_function).collect();
        if mapped_vec.contains(&expected_value) {
            return self;
        }

        panic_unmatched(self, expected_value, mapped_vec);
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

fn panic_unmatched<T, E: Debug, A: Debug>(spec: &mut Spec<T>, expected: E, actual: A) {
    spec.with_expected(format!("iterator to contain <{:?}>", expected))
        .with_actual(format!("<{:?}>", actual))
        .fail();
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;
    use std::collections::LinkedList;

    #[test]
    fn should_not_panic_if_vec_contains_value() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).contains(&2);
    }

    #[test]
    #[should_panic(expected = "expected iterator to contain <5> but was <[1, 2, 3]>")]
    fn should_panic_if_vec_does_not_contain_value() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).contains(&5);
    }

    #[test]
    fn should_not_panic_if_iterator_contains_value() {
        let mut test_into_iter = LinkedList::new();
        test_into_iter.push_back(1);
        test_into_iter.push_back(2);
        test_into_iter.push_back(3);

        assert_that(&test_into_iter).contains(&2);
    }

    #[test]
    #[should_panic(expected = "expected iterator to contain <5> but was <[1, 2, 3]>")]
    fn should_panic_if_iterator_does_not_contain_value() {
        let mut test_into_iter = LinkedList::new();
        test_into_iter.push_back(1);
        test_into_iter.push_back(2);
        test_into_iter.push_back(3);

        assert_that(&test_into_iter).contains(&5);
    }

    #[test]
    fn should_not_panic_if_iterator_matches_on_value() {
        let mut test_into_iter = LinkedList::new();
        test_into_iter.push_back(TestEnum::Bad);
        test_into_iter.push_back(TestEnum::Good);
        test_into_iter.push_back(TestEnum::Bad);

        assert_that(&test_into_iter).matching_contains(|val| {
            match val {
                &TestEnum::Good => true,
                _ => false,
            }
        });
    }

    #[test]
    #[should_panic(expected = "expectation failed for iterator with values <[Bad, Bad, Bad]>")]
    fn should_panic_if_iterator_does_not_match_on_value() {
        let mut test_into_iter = LinkedList::new();
        test_into_iter.push_back(TestEnum::Bad);
        test_into_iter.push_back(TestEnum::Bad);
        test_into_iter.push_back(TestEnum::Bad);

        assert_that(&test_into_iter).matching_contains(|val| {
            match val {
                &TestEnum::Good => true,
                _ => false,
            }
        });
    }

    #[test]
    fn should_not_panic_if_vec_contains_mapped_value() {
        let test_vec = vec![TestStruct { value: 5 }, TestStruct { value: 6 }];
        assert_that(&test_vec).mapped_contains(|val| val.value, &5);
    }

    #[test]
    #[should_panic(expected = "expected iterator to contain <1> but was <[5, 6]>")]
    fn should_panic_if_vec_does_not_contain_mapped_value() {
        let test_vec = vec![TestStruct { value: 5 }, TestStruct { value: 6 }];
        assert_that(&test_vec).mapped_contains(|val| val.value, &1);
    }

    #[derive(Debug, PartialEq)]
    struct TestStruct {
        pub value: u8,
    }

    #[derive(Debug)]
    enum TestEnum {
        Good,
        Bad,
    }

}
