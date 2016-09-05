use std::cmp::PartialEq;
use std::fmt::Debug;

mod option;
mod result;
mod vec;

#[derive(Debug)]
pub struct Spec<'s, S: 's> {
    subject: &'s S,
}

pub fn assert_that<'s, S>(subject: &'s S) -> Spec<'s, S> {
    Spec { subject: subject }
}

fn build_expectation_string(expected: &str, actual: &str) -> String {
    format!("expected {} but was {}", expected, actual)
}

impl<'s, S> Spec<'s, S>
    where S: Debug + PartialEq
{
    pub fn is_equal_to(&self, expected: &S) {
        if !self.subject.eq(expected) {
            panic!(build_expectation_string(&format!("{:?}", expected),
                                            &format!("{:?}", self.subject)));
        }
    }
}

impl<'s, S> Spec<'s, S>
    where S: Debug
{
    pub fn matches<F>(&self, matching_function: F)
        where F: Fn(&'s S) -> bool
    {
        if !matching_function(self.subject) {
            panic!(format!("assertion failed on value of <{:?}>", self.subject));
        }
    }

    pub fn map<F, T>(self, mapping_function: F) -> Spec<'s, T>
        where F: Fn(&'s S) -> &'s T
    {
        Spec { subject: mapping_function(self.subject) }
    }
}
