use std::cmp::PartialEq;
use std::fmt::Debug;

pub mod numeric;
pub mod option;
pub mod prelude;
pub mod result;
pub mod string;
pub mod vec;
pub mod iter;

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
    pub fn with_expected(&mut self, expected: String) -> &mut Self {
        let mut spec = self;
        spec.expected = Some(expected);
        spec
    }

    pub fn with_actual(&mut self, actual: String) -> &mut Self {
        let mut spec = self;
        spec.actual = Some(actual);
        spec
    }

    pub fn fail(&mut self) {
        if !self.expected.is_some() || !self.actual.is_some() {
            panic!("invalid assertion");
        }

        match self.description {
            Some(description) => {
                panic!(format!("{}: expected {} but was {}",
                               description,
                               self.expected.clone().unwrap(),
                               self.actual.clone().unwrap()))
            }
            None => {
                panic!(format!("expected {} but was {}",
                               self.expected.clone().unwrap(),
                               self.actual.clone().unwrap()))
            }
        }
    }

    fn fail_with_message(&mut self, message: String) {
        match self.description {
            Some(description) => panic!(format!("{}: {}", description, message)),
            None => panic!(message),
        }
    }
}

impl<'s, S> Spec<'s, S>
    where S: Debug + PartialEq
{
    pub fn is_equal_to(&mut self, expected: &S) -> &mut Self {
        let subject = self.subject;

        if !subject.eq(expected) {
            self.with_expected(format!("<{:?}>", expected))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }

        self
    }
}

impl<'s, S> Spec<'s, S>
    where S: Debug
{
    pub fn matches<F>(&mut self, matching_function: F) -> &mut Self
        where F: Fn(&'s S) -> bool
    {
        let subject = self.subject;

        if !matching_function(subject) {
            self.fail_with_message(format!("expectation failed for value <{:?}>", subject));
        }

        self
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
