use super::Spec;

use std::cmp::PartialEq;
use std::fmt::Debug;

pub trait VecSpec {
    fn has_length(&mut self, expected: usize) -> &mut Self;
}

impl<'s, T> VecSpec for Spec<'s, Vec<T>> {
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

pub trait MappingComparingVecSpec<'s, T: 's>
    where T: Debug
{
    fn mapped_contains<F, M: 's>(&mut self, mapping_function: F, expected_value: &M) -> &mut Self
        where M: Debug + PartialEq,
              F: Fn(&'s T) -> M;
}

impl<'s, T> MappingComparingVecSpec<'s, T> for Spec<'s, Vec<T>>
    where T: Debug
{
    fn mapped_contains<F, M: 's>(&mut self, mapping_function: F, expected_value: &M) -> &mut Self
        where M: Debug + PartialEq,
              F: Fn(&'s T) -> M
    {
        let subject = self.subject;

        let mapped_vec: Vec<M> = subject.iter().map(mapping_function).collect();
        if !mapped_vec.contains(&expected_value) {
            self.panic_unmatched(expected_value, mapped_vec);
        }

        self
    }
}

impl<'s, T> Spec<'s, Vec<T>> {
    fn panic_unmatched<E: Debug, A: Debug>(&mut self, expected: E, actual: A) {
        self.with_expected(format!("vec to contain <{:?}>", expected))
            .with_actual(format!("<{:?}>", actual))
            .fail();
    }
}
