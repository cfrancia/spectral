use std::cmp::PartialEq;
use std::fmt::Display;

#[derive(Debug)]
pub struct Spec<'s, S: 's> {
    pub subject: &'s S,
}

impl<'s, S> Spec<'s, S> {
    pub fn assert_that(subject: &'s S) -> Spec<'s, S> {
        Spec { subject: subject }
    }
}

impl<'s, S> Spec<'s, S> where S: Display + PartialEq {
    pub fn is_equal_to(&self, expected: &S) {
        if !self.subject.eq(expected) {
            panic!(format!("expected <{}> but was <{}>", expected, self.subject));
        }
    }
}

impl<'s, T> Spec<'s, Vec<T>> {
    pub fn has_length(&self, expected: usize) {
        let length = self.subject.len();
        if length != expected {
            panic!(format!("expected vec with length of <{}> but was <{}>", expected, length));
        }
    }
}

impl<'s, T> Spec<'s, Option<T>> where T: Display + PartialEq {
    pub fn contains_value(&self, expected_value: &T) {
        match self.subject {
            &Some(ref val) => {
                if !val.eq(expected_value) {
                    panic!(build_failure_string(&format!("<{}>", expected_value), &format!("<{}>", val)));
                }
            },
            &None => panic!(build_failure_string(&format!("<{}>", expected_value), "empty"))
        };

        fn build_failure_string(containing: &str, actual: &str) -> String {
            format!("expected option containing {} but was {}", containing, actual)
        }
    }

    pub fn is_some(&self) {
        match self.subject {
            &None => panic!(format!("expected non-empty option but was empty")),
            &Some(_) => ()
        };
    }

    pub fn is_none(&self) {
        match self.subject {
            &None => (),
            &Some(ref val) => panic!(format!("expected empty option but contained <{}>", val))
        };
    }
}

impl<'s, S> Spec<'s, S> {
    pub fn map<F, T>(self, mapping_function: F) -> Spec<'s, T>
        where F: Fn(&'s S) -> &'s T {
            Spec { subject: mapping_function(self.subject) }
        }
}
