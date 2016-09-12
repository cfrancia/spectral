use super::Spec;

use std::fmt::Debug;
use std::cmp::PartialOrd;

pub trait OrderedSpec<T>
    where T: Debug + PartialOrd
{
    fn is_less_than(self, other: &T);
    fn is_less_than_or_equal_to(self, other: &T);
    fn is_greater_than(self, other: &T);
    fn is_greater_than_or_equal_to(self, other: &T);
}

impl<'s, T> OrderedSpec<T> for Spec<'s, T>
    where T: Debug + PartialOrd
{
    fn is_less_than(self, other: &T) {
        let subject = self.subject;

        if subject >= other {
            self.with_expected(format!("value less than <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }

    fn is_less_than_or_equal_to(self, other: &T) {
        let subject = self.subject;

        if subject > other {
            self.with_expected(format!("value less than or equal to <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }

    fn is_greater_than(self, other: &T) {
        let subject = self.subject;

        if subject <= other {
            self.with_expected(format!("value greater than <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }

    fn is_greater_than_or_equal_to(self, other: &T) {
        let subject = self.subject;

        if subject < other {
            self.with_expected(format!("value greater than or equal to <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }
}
