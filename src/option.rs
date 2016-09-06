use super::Spec;

use std::cmp::PartialEq;
use std::fmt::Debug;

pub trait OptionSpec<T>
    where T: Debug
{
    fn is_some(self);
    fn is_none(self);
}

pub trait ComparingOptionSpec<T>
    where T: Debug + PartialEq
{
    fn contains_value(self, expected_value: &T);
}

impl<'s, T> ComparingOptionSpec<T> for Spec<'s, Option<T>>
    where T: Debug + PartialEq
{
    fn contains_value(self, expected_value: &T) {
        match self.subject {
            &Some(ref val) => {
                if !val.eq(expected_value) {
                    self.with_expected(format!("option to contain <{:?}>", expected_value))
                        .with_actual(format!("<{:?}>", val))
                        .fail();
                }
            }
            &None => {
                self.with_expected(format!("option<{:?}>", expected_value))
                    .with_actual(format!("option[none]"))
                    .fail();
            }
        };
    }
}

impl<'s, T> OptionSpec<T> for Spec<'s, Option<T>>
    where T: Debug
{
    fn is_some(self) {
        match self.subject {
            &Some(_) => (),
            &None => {
                self.with_expected(format!("option[some]"))
                    .with_actual(format!("option[none]"))
                    .fail();
            }
        };
    }

    fn is_none(self) {
        match self.subject {
            &None => (),
            &Some(ref val) => {
                self.with_expected(format!("option[none]"))
                    .with_actual(format!("option<{:?}>", val))
                    .fail();
            }
        };
    }
}
