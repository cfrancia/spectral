use super::{build_expectation_string, Spec};

use std::cmp::PartialEq;
use std::fmt::Debug;

impl<'s, T> Spec<'s, Option<T>>
    where T: Debug + PartialEq
{
    pub fn contains_value(&self, expected_value: &T) {
        match self.subject {
            &Some(ref val) => {
                if !val.eq(expected_value) {
                    panic!(build_expectation_string(&format!("option to contain <{:?}>",
                                                             expected_value),
                                                    &format!("<{:?}>", val)));
                }
            }
            &None => {
                panic!(build_expectation_string(&format!("option<{:?}>", expected_value),
                                                &"option[none]"))
            }
        };
    }

    pub fn is_some(&self) {
        match self.subject {
            &Some(_) => (),
            &None => panic!(build_expectation_string(&"option[some]", &"option[none]")),
        };
    }

    pub fn is_none(&self) {
        match self.subject {
            &None => (),
            &Some(ref val) => {
                panic!(build_expectation_string(&"option[none]", &format!("option<{:?}>", val)))
            }
        };
    }
}
