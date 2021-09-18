use super::{AssertionFailure, Spec};

use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait HashMapAssertions<'s, K: Hash + Eq, V: PartialEq> {
    fn has_length(&mut self, expected: usize);
    fn is_empty(&mut self);
    fn contains_key<E: Borrow<K>>(&mut self, expected_key: E) -> Spec<'s, V>;
    fn does_not_contain_key<E: Borrow<K>>(&mut self, expected_key: E);
    fn contains_entry<E: Borrow<K>, F: Borrow<V>>(&mut self, expected_key: E, expected_value: F);
    fn does_not_contain_entry<E: Borrow<K>, F: Borrow<V>>(
        &mut self,
        expected_key: E,
        expected_value: F,
    );
}

impl<'s, K, V> HashMapAssertions<'s, K, V> for Spec<'s, HashMap<K, V>>
where
    K: Hash + Eq + Debug,
    V: PartialEq + Debug,
{
    /// Asserts that the length of the subject hashmap is equal to the provided length. The subject
    /// type must be of `HashMap`.
    ///
    /// ```rust
    /// # use spectral::prelude::*;
    /// # use std::collections::HashMap;
    /// let mut test_map = HashMap::new();
    /// test_map.insert(1, 1);
    /// test_map.insert(2, 2);
    ///
    /// assert_that(&test_map).has_length(2);
    /// ```
    fn has_length(&mut self, expected: usize) {
        let subject = self.subject;

        if subject.len() != expected {
            AssertionFailure::from_spec(self)
                .with_expected(format!("hashmap to have length <{}>", expected))
                .with_actual(format!("<{}>", subject.len()))
                .fail();
        }
    }

    /// Asserts that the subject hashmap is empty. The subject type must be of `HashMap`.
    ///
    /// ```rust
    /// # use spectral::prelude::*;
    /// # use std::collections::HashMap;
    /// let test_map: HashMap<u8, u8> = HashMap::new();
    /// assert_that(&test_map).is_empty();
    /// ```
    fn is_empty(&mut self) {
        let subject = self.subject;

        if !subject.is_empty() {
            AssertionFailure::from_spec(self)
                .with_expected(format!("an empty hashmap"))
                .with_actual(format!("a hashmap with length <{:?}>", subject.len()))
                .fail();
        }
    }

    /// Asserts that the subject hashmap contains the expected key. The subject type must be
    /// of `HashMap`.
    ///
    /// This will return a new `Spec` containing the associated value if the key is present.
    ///
    /// ```rust
    /// # use spectral::prelude::*;
    /// # use std::collections::HashMap;
    /// let mut test_map = HashMap::new();
    /// test_map.insert("hello", "hi");
    ///
    /// assert_that(&test_map).contains_key(&"hello");
    /// ```
    fn contains_key<E: Borrow<K>>(&mut self, expected_key: E) -> Spec<'s, V> {
        let subject = self.subject;
        let borrowed_expected_key = expected_key.borrow();

        if let Some(value) = subject.get(borrowed_expected_key) {
            return Spec {
                subject: value,
                subject_name: self.subject_name,
                location: self.location.clone(),
                description: self.description,
            };
        }

        let subject_keys: Vec<&K> = subject.keys().collect();

        AssertionFailure::from_spec(self)
            .with_expected(format!(
                "hashmap to contain key <{:?}>",
                borrowed_expected_key
            ))
            .with_actual(format!("<{:?}>", subject_keys))
            .fail();

        unreachable!();
    }

    /// Asserts that the subject hashmap does not contain the provided key. The subject type must be
    /// of `HashMap`.
    ///
    /// ```rust
    /// # use spectral::prelude::*;
    /// # use std::collections::HashMap;
    /// let mut test_map = HashMap::new();
    /// test_map.insert("hello", "hi");
    ///
    /// assert_that(&test_map).does_not_contain_key(&"hey");
    /// ```
    fn does_not_contain_key<E: Borrow<K>>(&mut self, expected_key: E) {
        let subject = self.subject;
        let borrowed_expected_key = expected_key.borrow();

        if subject.get(borrowed_expected_key).is_some() {
            AssertionFailure::from_spec(self)
                .with_expected(format!(
                    "hashmap to not contain key <{:?}>",
                    borrowed_expected_key
                ))
                .with_actual(format!("present in hashmap"))
                .fail();
        }
    }

    /// Asserts that the subject hashmap contains the expected key with the expected value.
    /// The subject type must be of `HashMap`.
    ///
    /// ```rust
    /// # use spectral::prelude::*;
    /// # use std::collections::HashMap;
    /// let mut test_map = HashMap::new();
    /// test_map.insert("hello", "hi");
    ///
    /// assert_that(&test_map).contains_entry(&"hello", &"hi");
    /// ```
    fn contains_entry<E: Borrow<K>, F: Borrow<V>>(&mut self, expected_key: E, expected_value: F) {
        let subject = self.subject;
        let borrowed_expected_key = expected_key.borrow();
        let borrowed_expected_value = expected_value.borrow();

        let expected_message = format!(
            "hashmap containing key <{:?}> with value <{:?}>",
            borrowed_expected_key, borrowed_expected_value
        );

        if let Some(value) = subject.get(borrowed_expected_key) {
            if value.eq(borrowed_expected_value) {
                return;
            }

            AssertionFailure::from_spec(self)
                .with_expected(expected_message)
                .with_actual(format!(
                    "key <{:?}> with value <{:?}> instead",
                    borrowed_expected_key, value
                ))
                .fail();

            unreachable!();
        }

        let subject_keys: Vec<&K> = subject.keys().collect();

        AssertionFailure::from_spec(self)
            .with_expected(expected_message)
            .with_actual(format!("no matching key, keys are <{:?}>", subject_keys))
            .fail();
    }

    /// Asserts that the subject hashmap does not contains the provided key and value.
    /// The subject type must be of `HashMap`.
    ///
    /// ```rust
    /// # use spectral::prelude::*;
    /// # use std::collections::HashMap;
    /// let mut test_map = HashMap::new();
    /// test_map.insert("hello", "hi");
    ///
    /// assert_that(&test_map).does_not_contain_entry(&"hello", &"hey");
    /// ```
    fn does_not_contain_entry<E: Borrow<K>, F: Borrow<V>>(
        &mut self,
        expected_key: E,
        expected_value: F,
    ) {
        let subject = self.subject;
        let borrowed_expected_key = expected_key.borrow();
        let borrowed_expected_value = expected_value.borrow();

        if let Some(value) = subject.get(borrowed_expected_key) {
            if !value.eq(borrowed_expected_value) {
                return;
            }

            AssertionFailure::from_spec(self)
                .with_expected(format!(
                    "hashmap to not contain key <{:?}> with value <{:?}>",
                    borrowed_expected_key, borrowed_expected_value
                ))
                .with_actual(format!("present in hashmap"))
                .fail();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;

    use std::collections::HashMap;

    #[test]
    fn should_not_panic_if_hashmap_length_matches_expected() {
        let mut test_map = HashMap::new();
        test_map.insert(1, 1);
        test_map.insert(2, 2);

        assert_that(&test_map).has_length(2);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: hashmap to have length <1>\n\t but was: <2>")]
    fn should_panic_if_hashmap_length_does_not_match_expected() {
        let mut test_map = HashMap::new();
        test_map.insert(1, 1);
        test_map.insert(2, 2);

        assert_that(&test_map).has_length(1);
    }

    #[test]
    fn should_not_panic_if_hashmap_was_expected_to_be_empty_and_is() {
        let test_map: HashMap<u8, u8> = HashMap::new();
        assert_that(&test_map).is_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: an empty hashmap\
                   \n\t but was: a hashmap with length <1>")]
    fn should_panic_if_hashmap_was_expected_to_be_empty_and_is_not() {
        let mut test_map = HashMap::new();
        test_map.insert(1, 1);

        assert_that(&test_map).is_empty();
    }

    #[test]
    fn contains_key_should_allow_multiple_borrow_forms() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map).contains_key("hello");
        assert_that(&test_map).contains_key(&mut "hello");
        assert_that(&test_map).contains_key(&"hello");
    }

    #[test]
    fn should_not_panic_if_hashmap_contains_key() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map).contains_key(&"hello");
    }

    #[test]
    // Unfortunately the order of the keys can change. Doesn't seem to make sense to sort them
    // just for the sake of checking the panic message.
    #[should_panic]
    fn should_not_panic_if_hashmap_does_not_contain_key() {
        let mut test_map = HashMap::new();
        test_map.insert("hi", "hi");
        test_map.insert("hey", "hey");

        assert_that(&test_map).contains_key(&"hello");
    }

    #[test]
    fn should_be_able_to_chain_value_from_contains_key() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map)
            .contains_key(&"hello")
            .is_equal_to(&"hi");
    }

    #[test]
    fn does_not_contain_key_should_allow_multiple_borrow_forms() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map).does_not_contain_key("hey");
        assert_that(&test_map).does_not_contain_key(&mut "hey");
        assert_that(&test_map).does_not_contain_key(&"hey");
    }

    #[test]
    fn should_not_panic_if_hashmap_does_not_contain_key_when_expected() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map).does_not_contain_key(&"hey");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: hashmap to not contain key <\"hello\">\
                   \n\t but was: present in hashmap")]
    fn should_panic_if_hashmap_does_contain_key_when_not_expected() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map).does_not_contain_key(&"hello");
    }

    #[test]
    fn contains_entry_should_allow_multiple_borrow_forms() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map).contains_entry("hello", "hi");
        assert_that(&test_map).contains_entry(&mut "hello", &mut "hi");
        assert_that(&test_map).contains_entry("hello", &mut "hi");
        assert_that(&test_map).contains_entry(&"hello", &"hi");
    }

    #[test]
    fn should_not_panic_if_hashmap_contains_entry() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map).contains_entry(&"hello", &"hi");
    }

    #[test]
    #[should_panic(
        expected = "\n\texpected: hashmap containing key <\"hey\"> with value <\"hi\">\
                   \n\t but was: no matching key, keys are <[\"hello\"]>"
    )]
    fn should_panic_if_hashmap_contains_entry_without_key() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map).contains_entry(&"hey", &"hi");
    }

    #[test]
    #[should_panic(
        expected = "\n\texpected: hashmap containing key <\"hi\"> with value <\"hey\">\
                   \n\t but was: key <\"hi\"> with value <\"hello\"> instead"
    )]
    fn should_panic_if_hashmap_contains_entry_with_different_value() {
        let mut test_map = HashMap::new();
        test_map.insert("hi", "hello");

        assert_that(&test_map).contains_entry(&"hi", &"hey");
    }

    #[test]
    fn should_not_panic_if_hashmap_does_not_contain_entry_if_expected() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map).does_not_contain_entry(&"hey", &"hi");
    }

    #[test]
    fn should_not_panic_if_hashmap_contains_entry_with_different_value_if_expected() {
        let mut test_map = HashMap::new();
        test_map.insert("hi", "hello");

        assert_that(&test_map).does_not_contain_entry(&"hi", &"hey");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: hashmap to not contain key <\"hello\"> \
    with value <\"hi\">\
                   \n\t but was: present in hashmap")]
    fn should_panic_if_hashmap_contains_entry_if_not_expected() {
        let mut test_map = HashMap::new();
        test_map.insert("hello", "hi");

        assert_that(&test_map).does_not_contain_entry(&"hello", &"hi");
    }
}
