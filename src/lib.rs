use std::cmp::PartialEq;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Spec<'s, S: 's> {
    subject: &'s S,
}

pub fn assert_that<'s, S>(subject: &'s S) -> Spec<'s, S> {
    Spec { subject: subject }
}

impl<'s, S> Spec<'s, S>
    where S: Debug + PartialEq
{
    pub fn is_equal_to(&self, expected: &S) {
        if !self.subject.eq(expected) {
            panic!(format!("expected <{:?}> but was <{:?}>", expected, self.subject));
        }
    }
}

impl<'s, T> Spec<'s, Vec<T>>
    where T: Debug + PartialEq
{
    pub fn has_length(&self, expected: usize) {
        let length = self.subject.len();
        if length != expected {
            panic!(format!("expected vec with length of <{:?}> but was <{:?}>",
                           expected,
                           length));
        }
    }

    pub fn contains(&self, expected_value: &T) {
        if !self.subject.contains(expected_value) {
            Self::panic_unmatched(expected_value, self.subject);
        }
    }

    pub fn mapped_contains<F, M: 's>(&self, mapping_function: F, expected_value: &M)
        where M: Debug + PartialEq,
              F: Fn(&'s T) -> &M
    {
        let mapped_vec: Vec<&M> = self.subject.iter().map(mapping_function).collect();
        if !mapped_vec.contains(&expected_value) {
            Self::panic_unmatched(expected_value, mapped_vec);
        }
    }

    fn panic_unmatched<E: Debug, A: Debug>(expected: E, actual: A) {
        panic!(format!("expected vec to contain <{:?}> but contains <{:?}>",
                       expected,
                       actual));
    }
}

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

impl<'s, T, E> Spec<'s, Result<T, E>>
    where T: Debug,
          E: Debug
{
    pub fn is_ok(&self) {
        match self.subject {
            &Ok(_) => (),
            &Err(ref err) => {
                panic!(format!("expected ok result but was error result of <{:?}>", err))
            }
        };
    }

    pub fn is_error(&self) {
        match self.subject {
            &Err(_) => (),
            &Ok(ref val) => {
                panic!(format!("expected error result but was ok result of <{:?}>", val))
            }
        };
    }
}

impl<'s, S> Spec<'s, S>
    where S: Debug
{
    pub fn matches<F>(&self, matching_function: F)
        where F: Fn(&'s S) -> bool
    {
        if !matching_function(self.subject) {
            panic!(format!("assertion failed on value of <{:?}>", self.subject));
        }
    }

    pub fn map<F, T>(self, mapping_function: F) -> Spec<'s, T>
        where F: Fn(&'s S) -> &'s T
    {
        Spec { subject: mapping_function(self.subject) }
    }
}
