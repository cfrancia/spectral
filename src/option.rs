use super::Spec;

use std::cmp::PartialEq;
use std::fmt::Debug;

pub trait OptionSpec<T>
    where T: Debug
{
    fn is_some(&mut self) -> &mut Self;
    fn is_none(&mut self) -> &mut Self;
}

pub trait ComparingOptionSpec<T>
    where T: Debug + PartialEq
{
    fn contains_value(&mut self, expected_value: &T) -> &mut Self;
}

impl<'s, T> ComparingOptionSpec<T> for Spec<'s, Option<T>>
    where T: Debug + PartialEq
{
    fn contains_value(&mut self, expected_value: &T) -> &mut Self {
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

        self
    }
}

impl<'s, T> OptionSpec<T> for Spec<'s, Option<T>>
    where T: Debug
{
    fn is_some(&mut self) -> &mut Self {
        match self.subject {
            &Some(_) => (),
            &None => {
                self.with_expected(format!("option[some]"))
                    .with_actual(format!("option[none]"))
                    .fail();
            }
        };

        self
    }

    fn is_none(&mut self) -> &mut Self {
        match self.subject {
            &None => (),
            &Some(ref val) => {
                self.with_expected(format!("option[none]"))
                    .with_actual(format!("option<{:?}>", val))
                    .fail();
            }
        };

        self
    }
}
