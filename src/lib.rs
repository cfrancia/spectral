use std::cmp::PartialEq;
use std::fmt::Display;

#[derive(Debug)]
pub struct Spec<'s, S: 's + Display> {
    pub subject: &'s S,
}

impl<'s, S> Spec<'s, S> where S: Display {
    pub fn assert_that(subject: &'s S) -> Spec<'s, S> {
        Spec { subject: subject }
    }
}

impl<'s, S> Spec<'s, S> where S: Display + PartialEq {
    pub fn is_equal_to(&self, actual: &S) {
        if !self.subject.eq(actual) {
            panic!(format!("expected <{}> but was <{}>", actual, self.subject));
        }
    }
}
