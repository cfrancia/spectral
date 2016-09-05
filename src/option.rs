use super::{build_expectation_string, Spec};

use std::cmp::PartialEq;
use std::fmt::Debug;

pub trait OptionSpec<T>
    where T: Debug
{
    fn is_some(&self);
    fn is_none(&self);
}

pub trait ComparingOptionSpec<T>
    where T: Debug + PartialEq
{
    fn contains_value(&self, expected_value: &T);
}

impl<'s, T> ComparingOptionSpec<T> for Spec<'s, Option<T>>
    where T: Debug + PartialEq
{
    fn contains_value(&self, expected_value: &T) {
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
}

impl<'s, T> OptionSpec<T> for Spec<'s, Option<T>>
    where T: Debug
{
    fn is_some(&self) {
        match self.subject {
            &Some(_) => (),
            &None => panic!(build_expectation_string(&"option[some]", &"option[none]")),
        };
    }

    fn is_none(&self) {
        match self.subject {
            &None => (),
            &Some(ref val) => {
                panic!(build_expectation_string(&"option[none]", &format!("option<{:?}>", val)))
            }
        };
    }
}
