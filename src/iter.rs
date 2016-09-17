use super::Spec;

use std::cmp::PartialEq;
use std::fmt::Debug;

pub trait ComparingIterSpec<'s, T: 's>
    where T: Debug + PartialEq
{
    fn contains(self, expected_value: &T);
}

impl<'s, T: 's, I> ComparingIterSpec<'s, T> for Spec<'s, I>
    where T: Debug + PartialEq,
          &'s I: IntoIterator<Item=&'s T>
{
    fn contains(self, expected_value: &T) {
        let mut actual = Vec::new();
        for x in self.subject {
            if expected_value.eq(x) {
                return;
            } else {
                actual.push(x);
            }
        }
        self.with_expected(format!("iterator to contain <{:?}>", expected_value))
            .with_actual(format!("<{:?}>", actual))
            .fail();
    }
}
