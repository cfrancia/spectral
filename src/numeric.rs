use super::Spec;

use std::fmt::Debug;
use std::cmp::PartialOrd;

pub trait OrderedSpec<T>
    where T: Debug + PartialOrd
{
    fn is_less_than(&mut self, other: &T) -> &mut Self;
    fn is_less_than_or_equal_to(&mut self, other: &T) -> &mut Self;
    fn is_greater_than(&mut self, other: &T) -> &mut Self;
    fn is_greater_than_or_equal_to(&mut self, other: &T) -> &mut Self;
}

impl<'s, T> OrderedSpec<T> for Spec<'s, T>
    where T: Debug + PartialOrd
{
    fn is_less_than(&mut self, other: &T) -> &mut Self {
        let subject = self.subject;

        if subject >= other {
            self.with_expected(format!("value less than <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }

        self
    }

    fn is_less_than_or_equal_to(&mut self, other: &T) -> &mut Self {
        let subject = self.subject;

        if subject > other {
            self.with_expected(format!("value less than or equal to <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }

        self
    }

    fn is_greater_than(&mut self, other: &T) -> &mut Self {
        let subject = self.subject;

        if subject <= other {
            self.with_expected(format!("value greater than <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }

        self
    }

    fn is_greater_than_or_equal_to(&mut self, other: &T) -> &mut Self {
        let subject = self.subject;

        if subject < other {
            self.with_expected(format!("value greater than or equal to <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }

        self
    }
}
