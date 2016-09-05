use super::Spec;

use std::cmp::PartialEq;
use std::fmt::Debug;

impl<'s, T> Spec<'s, Option<T>>
    where T: Debug + PartialEq
{
    pub fn contains_value(&self, expected_value: &T) {
        match self.subject {
            &Some(ref val) => {
                if !val.eq(expected_value) {
                    panic!(build_failure_string(&format!("<{:?}>", expected_value),
                                                &format!("<{:?}>", val)));
                }
            }
            &None => panic!(build_failure_string(&format!("<{:?}>", expected_value), "empty")),
        };

        fn build_failure_string(containing: &str, actual: &str) -> String {
            format!("expected option containing {} but was {}",
                    containing,
                    actual)
        }
    }

    pub fn is_some(&self) {
        match self.subject {
            &Some(_) => (),
            &None => panic!(format!("expected non-empty option but was empty")),
        };
    }

    pub fn is_none(&self) {
        match self.subject {
            &None => (),
            &Some(ref val) => panic!(format!("expected empty option but contained <{:?}>", val)),
        };
    }
}
