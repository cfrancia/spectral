use std::cmp::PartialEq;
use std::fmt::Debug;

pub mod option;
pub mod prelude;
pub mod result;
pub mod vec;

#[derive(Debug)]
pub struct SpecDescription<'r> {
    value: &'r str,
}

#[derive(Debug)]
pub struct Spec<'s, S: 's> {
    pub subject: &'s S,
    description: Option<&'s str>,
    expected: Option<String>,
    actual: Option<String>,
}

pub fn assert_that<'s, S>(subject: &'s S) -> Spec<'s, S> {
    Spec {
        subject: subject,
        description: None,
        expected: None,
        actual: None,
    }
}

pub fn asserting<'r>(description: &'r str) -> SpecDescription {
    SpecDescription { value: description }
}

fn build_expectation_string(expected: &str, actual: &str) -> String {
    format!("expected {} but was {}", expected, actual)
}

impl<'r> SpecDescription<'r> {
    pub fn that<S>(self, subject: &'r S) -> Spec<'r, S> {
        Spec {
            subject: subject,
            description: Some(self.value),
            expected: None,
            actual: None,
        }
    }
}

impl<'s, S> Spec<'s, S> {
    pub fn with_expected(self, expected: String) -> Self {
        let mut spec = self;
        spec.expected = Some(expected);
        spec
    }

    pub fn with_actual(self, actual: String) -> Self {
        let mut spec = self;
        spec.actual = Some(actual);
        spec
    }

    pub fn fail(self) {
        if !self.expected.is_some() || !self.actual.is_some() {
            panic!("invalid assertion");
        }

        match self.description {
            Some(description) => {
                panic!(format!("{}: expected {} but was {}",
                               description,
                               self.expected.unwrap(),
                               self.actual.unwrap()))
            }
            None => {
                panic!(format!("expected {} but was {}",
                               self.expected.unwrap(),
                               self.actual.unwrap()))
            }
        }
    }

    fn fail_with_message(self, message: String) {
        match self.description {
            Some(description) => panic!(format!("{}: {}", description, message)),
            None => panic!(message),
        }
    }
}

impl<'s, S> Spec<'s, S>
    where S: Debug + PartialEq
{
    pub fn is_equal_to(self, expected: &S) {
        let subject = self.subject;

        if !subject.eq(expected) {
            self.with_expected(format!("<{:?}>", expected))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }
}

impl<'s, S> Spec<'s, S>
    where S: Debug
{
    pub fn matches<F>(self, matching_function: F)
        where F: Fn(&'s S) -> bool
    {
        let subject = self.subject;

        if !matching_function(subject) {
            self.fail_with_message(format!("expectation failed for value <{:?}>", subject));
        }
    }

    pub fn map<F, T>(self, mapping_function: F) -> Spec<'s, T>
        where F: Fn(&'s S) -> &'s T
    {
        Spec {
            subject: mapping_function(self.subject),
            description: self.description,
            expected: self.expected,
            actual: self.actual,
        }
    }
}
