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
//! spectral = "0.4.0"
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

use colours::{TERM_RED, TERM_BOLD, TERM_RESET};

pub mod boolean;
pub mod hashmap;
pub mod numeric;
pub mod option;
pub mod path;
pub mod prelude;
pub mod result;
pub mod string;
pub mod vec;
pub mod iter;

// Disable colours during tests, otherwise trying to assert on the panic message becomes
// significantly more annoying.
#[cfg(not(test))]
mod colours {
    pub const TERM_RED: &'static str = "\x1B[31m";
    pub const TERM_BOLD: &'static str = "\x1B[1m";
    pub const TERM_RESET: &'static str = "\x1B[0m";
}

#[cfg(test)]
mod colours {
    pub const TERM_RED: &'static str = "";
    pub const TERM_BOLD: &'static str = "";
    pub const TERM_RESET: &'static str = "";
}

#[cfg(feature = "num")]
extern crate num;

#[macro_export]
macro_rules! assert_that {
    (&$subject:tt) => {
        assert_that!($subject)
    };
    ($subject:tt) => {
        {
            let line = line!();
            let file =  file!();
            assert_that(&$subject).at_location(format!("{}:{}", file, line))
        }
    };
    (&$subject:ident$(.$additional_subject:ident)*) => {
        assert_that!($subject$(.$additional_subject)*)
    };
    ($subject:ident$(.$additional_subject:ident)*) => {
        {
            let line = line!();
            let file =  file!();
            assert_that(&$subject$(.$additional_subject)*).at_location(format!("{}:{}", file, line))
        }
    };
}

#[macro_export]
macro_rules! asserting {
    (&$description:tt) => {
        asserting!($description)
    };
    ($description:tt) => {
        {
            let line = line!();
            let file =  file!();
            asserting(&$description).at_location(format!("{}:{}", file, line))
        }
    };
}

pub trait DescriptiveSpec<'r> {
    fn subject_name(&self) -> Option<&'r str>;
    fn location(&self) -> Option<String>;
    fn description(&self) -> Option<&'r str>;
}

/// A failed assertion.
///
/// This exposes builder methods to construct the final failure message.
#[derive(Debug)]
pub struct AssertionFailure<'r, T: 'r> {
    spec: &'r T,
    expected: Option<String>,
    actual: Option<String>,
}

/// A description for an assertion.
///
/// This is created by the `asserting` function.
#[derive(Debug)]
pub struct SpecDescription<'r> {
    value: &'r str,
    location: Option<String>,
}

/// An assertion.
///
/// This is created by either the `assert_that` function, or by calling `that` on a
/// `SpecDescription`.
#[derive(Debug)]
pub struct Spec<'s, S: 's> {
    pub subject: &'s S,
    pub subject_name: Option<&'s str>,
    pub location: Option<String>,
    pub description: Option<&'s str>,
}

/// Wraps a subject in a `Spec` to provide assertions against it.
///
/// The subject must be a reference.
pub fn assert_that<'s, S>(subject: &'s S) -> Spec<'s, S> {
    Spec {
        subject: subject,
        subject_name: None,
        location: None,
        description: None,
    }
}

/// Describes an assertion.
pub fn asserting(description: &str) -> SpecDescription {
    SpecDescription {
        value: description,
        location: None,
    }
}

impl<'r> SpecDescription<'r> {
    pub fn at_location(self, location: String) -> Self {
        let mut description = self;

        description.location = Some(location);
        description
    }

    /// Creates a new assertion, passing through its description.
    pub fn that<S>(self, subject: &'r S) -> Spec<'r, S> {
        Spec {
            subject: subject,
            subject_name: None,
            location: self.location,
            description: Some(self.value),
        }
    }
}

impl<'r, T> DescriptiveSpec<'r> for Spec<'r, T> {
    fn subject_name(&self) -> Option<&'r str> {
        self.subject_name
    }

    fn location(&self) -> Option<String> {
        self.location.clone()
    }

    fn description(&self) -> Option<&'r str> {
        self.description
    }
}

impl<'r, T: DescriptiveSpec<'r>> AssertionFailure<'r, T> {
    /// Construct a new AssertionFailure from a DescriptiveSpec.
    pub fn from_spec(spec: &'r T) -> AssertionFailure<'r, T> {
        AssertionFailure {
            spec: spec,
            expected: None,
            actual: None,
        }
    }

    /// Builder method to add the expected value for the panic message.
    pub fn with_expected(&mut self, expected: String) -> &mut Self {
        let mut assertion = self;
        assertion.expected = Some(expected);

        assertion
    }

    /// Builder method to add the actual value for the panic message.
    pub fn with_actual(&mut self, actual: String) -> &mut Self {
        let mut assertion = self;
        assertion.actual = Some(actual);

        assertion
    }

    /// Builds the failure message with a description (if present), the expected value,
    /// and the actual value and then calls `panic` with the created message.
    pub fn fail(&mut self) {
        if !self.expected.is_some() || !self.actual.is_some() {
            panic!("invalid assertion");
        }

        let location = self.maybe_build_location();
        let subject_name = self.maybe_build_subject_name();
        let description = self.maybe_build_description();

        panic!(format!("{}{}\n\t{}expected: {}\n\t but was: {}{}\n{}",
                       description,
                       subject_name,
                       TERM_RED,
                       self.expected.clone().unwrap(),
                       self.actual.clone().unwrap(),
                       TERM_RESET,
                       location))
    }

    /// Calls `panic` with the provided message, prepending the assertion description
    /// if present.
    fn fail_with_message(&mut self, message: String) {
        let location = self.maybe_build_location();
        let subject_name = self.maybe_build_subject_name();
        let description = self.maybe_build_description();

        panic!(format!("{}{}\n\t{}{}{}\n{}",
                       description,
                       subject_name,
                       TERM_RED,
                       message,
                       TERM_RESET,
                       location))
    }

    fn maybe_build_location(&self) -> String {
        match self.spec.location() {
            Some(value) => format!("\n\t{}at location: {}{}\n", TERM_BOLD, value, TERM_RESET),
            None => "".to_string(),
        }
    }

    fn maybe_build_description(&self) -> String {
        match self.spec.description() {
            Some(value) => format!("\n\t{}{}:{}", TERM_BOLD, value, TERM_RESET),
            None => "".to_string(),
        }
    }

    fn maybe_build_subject_name(&self) -> String {
        match self.spec.subject_name() {
            Some(value) => format!("\n\t{}for subject [{}]{}", TERM_BOLD, value, TERM_RESET),
            None => "".to_string(),
        }
    }
}

impl<'s, S> Spec<'s, S> {
    /// Provides the actual location of the assertion.
    ///
    /// Usually you would not call this directly, but use the macro forms of `assert_that` and
    /// `asserting`, which will call this on your behalf with the correct location.
    pub fn at_location(&mut self, location: String) -> &mut Self {
        self.location = Some(location);
        self
    }

    /// Associates a name with the subject.
    ///
    /// This will be displayed if the assertion fails.
    pub fn named(&mut self, subject_name: &'s str) -> &mut Self {
        self.subject_name = Some(subject_name);
        self
    }
}

impl<'s, S> Spec<'s, S>
    where S: Debug + PartialEq
{
    /// Asserts that the actual value and the expected value are equal. The value type must
    /// implement `PartialEq`.
    ///
    /// ```rust,ignore
    /// assert_that(&"hello").is_equal_to(&"hello");
    /// ```
    pub fn is_equal_to(&mut self, expected: &S) -> &mut Self {
        let subject = self.subject;

        if !subject.eq(expected) {
            AssertionFailure::from_spec(self)
                .with_expected(format!("<{:?}>", expected))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }

        self
    }

    /// Asserts that the actual value and the expected value are not equal. The value type must
    /// implement `PartialEq`.
    ///
    /// ```rust,ignore
    /// assert_that(&"hello").is_not_equal_to(&"hello");
    /// ```
    pub fn is_not_equal_to(&mut self, expected: &S) -> &mut Self {
        let subject = self.subject;

        if subject.eq(expected) {
            AssertionFailure::from_spec(self)
                .with_expected(format!("<{:?}> to not equal <{:?}>", subject, expected))
                .with_actual(format!("equal"))
                .fail();
        }

        self
    }
}

impl<'s, S> Spec<'s, S>
    where S: Debug
{
    /// Accepts a function accepting the value type which returns a bool. Returning false will
    /// cause the assertion to fail.
    ///
    /// NOTE: The resultant panic message will only state the actual value. It's recommended that
    /// you write your own assertion rather than relying upon this.
    ///
    /// ```rust,ignore
    /// assert_that(&"hello").matches(|x| x.eq(&"hello"));
    /// ```
    pub fn matches<F>(&mut self, matching_function: F)
        where F: Fn(&'s S) -> bool
    {
        let subject = self.subject;

        if !matching_function(subject) {
            AssertionFailure::from_spec(self)
                .fail_with_message(format!("expectation failed for value <{:?}>", subject));
        }
    }

    /// Transforms the subject of the `Spec` by passing it through to the provided mapping
    /// function.
    ///
    /// ```rust,ignore
    /// let test_struct = TestStruct { value: 5 };
    /// assert_that(&test_struct).map(|val| &val.value).is_equal_to(&5);
    /// ```
    pub fn map<F, T>(self, mapping_function: F) -> Spec<'s, T>
        where F: Fn(&'s S) -> &'s T
    {
        Spec {
            subject: mapping_function(self.subject),
            subject_name: self.subject_name,
            location: self.location.clone(),
            description: self.description,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::prelude::*;

    #[test]
    fn should_be_able_to_use_macro_form_with_deliberate_reference() {
        let test_vec = vec![1, 2, 3, 4, 5];

        assert_that!(&test_vec).mapped_contains(|val| val * 2, &6);
    }

    #[test]
    fn should_be_able_to_use_macro_form_without_deliberate_reference() {
        let test_vec = vec![1, 2, 3, 4, 5];

        assert_that!(test_vec).mapped_contains(|val| val * 2, &6);
    }

    #[test]
    #[should_panic(expected = "\n\ttest condition:\n\texpected: <2>\n\t but was: <1>")]
    fn should_contain_assertion_description_in_panic() {
        asserting(&"test condition").that(&1).is_equal_to(&2);
    }

    #[test]
    #[should_panic(expected = "\n\tclosure:\n\texpectation failed for value <\"Hello\">")]
    fn should_contain_assertion_description_if_message_is_provided() {
        let value = "Hello";
        asserting(&"closure").that(&value).matches(|val| val.eq(&"Hi"));
    }

    #[test]
    #[should_panic(expected = "\n\texpected: <2>\n\t but was: <1>\
                   \n\n\tat location: src/lib.rs:")]
    fn should_contain_file_and_line_in_panic_for_assertions() {
        assert_that!(&1).is_equal_to(&2);
    }

    #[test]
    #[should_panic(expected = "\n\texpectation failed for value <\"Hello\">\
                   \n\n\tat location: src/lib.rs:")]
    fn should_contain_file_and_line_for_assertions_if_message_is_provided() {
        let value = "Hello";
        assert_that!(&value).matches(|val| val.eq(&"Hi"));
    }

    #[test]
    #[should_panic(expected = "\n\ttest condition:\n\texpected: <2>\n\t but was: <1>\
                   \n\n\tat location: src/lib.rs:")]
    fn should_contain_file_and_line_in_panic_for_descriptive_assertions() {
        asserting!(&"test condition").that(&1).is_equal_to(&2);
    }

    #[test]
    #[should_panic(expected = "\n\tclosure:\n\texpectation failed for value <\"Hello\">\
                   \n\n\tat location: src/lib.rs:")]
    fn should_contain_file_and_line_for_descriptive_assertions_if_message_is_provided() {
        let value = "Hello";
        asserting!(&"closure").that(&value).matches(|val| val.eq(&"Hi"));
    }

    #[test]
    #[should_panic(expected = "\n\tfor subject [number one]\n\texpected: <2>\n\t but was: <1>\
                   \n\n\tat location: src/lib.rs:")]
    fn should_contain_subject_name_in_panic_for_assertions() {
        assert_that!(&1).named(&"number one").is_equal_to(&2);
    }

    #[test]
    #[should_panic(expected = "\n\tfor subject [a word]\n\texpectation failed for value <\"Hello\">\
                   \n\n\tat location: src/lib.rs:")]
    fn should_contain_subject_name_in_panic_for_assertions_if_message_is_provided() {
        let value = "Hello";
        assert_that!(&value).named(&"a word").matches(|val| val.eq(&"Hi"));
    }

    #[test]
    fn should_not_panic_on_equal_subjects() {
        assert_that(&1).is_equal_to(&1);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: <2>\n\t but was: <1>")]
    fn should_panic_on_unequal_subjects() {
        assert_that(&1).is_equal_to(&2);
    }

    #[test]
    fn should_not_panic_on_unequal_subjects_if_expected() {
        assert_that(&1).is_not_equal_to(&2);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: <1> to not equal <1>\n\t but was: equal")]
    fn should_panic_on_equal_subjects_if_expected_unequal() {
        assert_that(&1).is_not_equal_to(&1);
    }

    #[test]
    fn should_not_panic_if_value_matches() {
        let value = "Hello";
        assert_that(&value).matches(|val| val.eq(&"Hello"));
    }

    #[test]
    #[should_panic(expected = "\n\texpectation failed for value <\"Hello\">")]
    fn should_panic_if_value_does_not_match() {
        let value = "Hello";
        assert_that(&value).matches(|val| val.eq(&"Hi"));
    }

    #[test]
    fn should_be_able_to_map_to_inner_field_of_struct_when_matching() {
        let test_struct = TestStruct { value: 5 };
        assert_that(&test_struct).map(|val| &val.value).is_equal_to(&5);
    }

    #[derive(Debug, PartialEq)]
    struct TestStruct {
        pub value: u8,
    }

}
