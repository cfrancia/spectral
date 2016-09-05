use super::{build_expectation_string, Spec};

use std::cmp::PartialEq;
use std::fmt::Debug;

impl<'s, T> Spec<'s, Vec<T>> {
    pub fn has_length(&self, expected: usize) {
        let length = self.subject.len();
        if length != expected {
            panic!(build_expectation_string(&format!("vec to have length <{}>", expected),
                                            &format!("<{}>", length)));
        }
    }
}

impl<'s, T> Spec<'s, Vec<T>>
    where T: Debug + PartialEq
{
    pub fn contains(&self, expected_value: &T) {
        if !self.subject.contains(expected_value) {
            Self::panic_unmatched(expected_value, self.subject);
        }
    }

    pub fn mapped_contains<F, M: 's>(&self, mapping_function: F, expected_value: &M)
        where M: Debug + PartialEq,
              F: Fn(&'s T) -> &M
    {
        let mapped_vec: Vec<&M> = self.subject.iter().map(mapping_function).collect();
        if !mapped_vec.contains(&expected_value) {
            Self::panic_unmatched(expected_value, mapped_vec);
        }
    }

    fn panic_unmatched<E: Debug, A: Debug>(expected: E, actual: A) {
        panic!(build_expectation_string(&format!("vec to contain <{:?}>", expected),
                                        &format!("<{:?}>", actual)));
    }
}
