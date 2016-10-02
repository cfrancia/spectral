//! Fluent test assertions in Rust
//!
//! Spectral is a testing framework designed to make your assertions read like plain English.
//! This allows you to more easily expose the intent of your test, rather than having it shrouded by
//! assertions which work, but are opaque on their meaning.
//!
//! Methods available to assert with are dependent upon the type of the subject under test.
//! Assertions are available for some basic types, but there is still a great deal missing from the
//! standard library.
//!
//! ## Usage
//!
//! Add the dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! spectral = "0.2.0"
//! ```
//!
//! Then add this to your crate:
//!
//! ```rust
//! extern crate spectral;
//! ```
//!
//! If you want macro support, include `#[macro_use]` to the declaration:
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate spectral;
//! ```
//!
//! To quickly start using assertions, `use` the prelude module:
//!
//! ```rust
//! use spectral::prelude::*;
//! ```
//!
//! ## Example
//!
//! We're going to make a few assertions on a `String` we create. Normally you would
//! want to assert on the output of something, but we'll just pretend that something created it.
//!
//! First, we'll create a new test with our `String`.
//!
//! ```rust
//! #[test]
//! pub fn should_be_the_correct_string() {
//!     let subject = "Hello World!";
//! }
//! ```
//!
//! Note that it is good practice to make sure that you name your test in a way that actually
//! explains what it is trying to test. When you have a number of tests, and one of them fails,
//! something like this is easier to understand:
//!
//! ```rust,ignore
//! #[test]
//! pub fn should_return_false_if_condition_does_not_hold() {
//!     ...
//! }
//! ```
//!
//! Rather than if you have a test like this:
//!
//! ```rust,ignore
//! #[test]
//! pub fn should_work() {
//!     ...
//! }
//! ```
//!
//! Unfortunately, our test isn't named very well at the moment, but given the lack of context,
//! it'll have to do for now.
//!
//! Now that we have something to test, we need to actually start asserting on it. The first part
//! to that is to provide it to the `assert_that` function. Note that we need to provide it as a
//! reference.
//!
//! ```rust
//! #[test]
//! pub fn should_be_the_correct_string() {
//!     let subject = "Hello World!";
//!     assert_that(&subject);
//! }
//! ```
//!
//! If we run that with `cargo test`, we'll see the following output:
//!
//! ```bash
//! running 1 test
//! test should_be_the_correct_string ... ok
//! ```
//!
//! Our test compiles and passes, but we still haven't made any assertions. Let's make a simple one
//! to start with. We'll check to see that it starts with the letter 'H'.
//!
//! ```rust
//! #[test]
//! pub fn should_be_the_correct_string() {
//!     let subject = "Hello World!";
//!     assert_that(&subject).starts_with(&"H");
//! }
//! ```
//!
//! Once you run this, you'll notice that the test still passes. That's because we've just proven
//! something that was already true. Usually you'll want to start with a failing test, and then
//! change your code to make it pass, rather than writing the test after the implementation.
//!
//! But for the purpose of exploration, let's break the actual value. We'll change "Hello World!"
//! to be "ello World!".
//!
//! ```rust
//! #[test]
//! pub fn should_be_the_correct_string() {
//!     let subject = "ello World!";
//!     assert_that(&subject).starts_with(&"H");
//! }
//! ```
//!
//! This time, we see that the test fails, and we also get some output from our assertion to tell
//! us what it was, and what it was expected to be:
//!
//! ```bash
//! running 1 test
//! test should_be_the_correct_string ... FAILED
//!
//! failures:
//!
//! ---- should_be_the_correct_string stdout ----
//!     thread 'should_be_the_correct_string' panicked at 'expected string starting with <"H"> but
//!     was <"ello World!">', src/lib.rs:204
//! ```
//!
//! Great! So we've just encountered a failing test. This particular case is quite easy to fix up
//! (just add the letter 'H' back to the start of the `String`), but we can also see that the panic
//! message tells us enough information to work that out as well.
//!
//! Now, this was just a simple example, and there's a number of features not demonstrated, but
//! hopefully it's enough to start you off with writing assertions in your tests using Spectral.

use std::cmp::PartialEq;
use std::fmt::Debug;

pub mod numeric;
pub mod option;
pub mod prelude;
pub mod result;
pub mod string;
pub mod vec;
pub mod iter;

#[macro_export]
macro_rules! assert_that {
    (&$subject:ident$(.$additional_subject:ident)*) => {
        assert_that!($subject$(.$additional_subject)*)
    };
    ($subject:ident$(.$additional_subject:ident)*) => {
        assert_that(&$subject$(.$additional_subject)*)
    };
}

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
