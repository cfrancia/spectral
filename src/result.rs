use super::Spec;

use std::fmt::Debug;

pub trait ResultSpec<T, E>
    where T: Debug,
          E: Debug
{
    fn is_ok(&mut self) -> &mut Self;
    fn is_error(&mut self) -> &mut Self;
}

impl<'s, T, E> ResultSpec<T, E> for Spec<'s, Result<T, E>>
    where T: Debug,
          E: Debug
{
    /// Asserts that the subject is `Ok`. The value type must be a `Result`.
    ///
    /// ```rust,ignore
    /// assert_that(&Result::Ok::<usize, usize>(1)).is_ok();
    /// ```
    fn is_ok(&mut self) -> &mut Self {
        match self.subject {
            &Ok(_) => (),
            &Err(ref err) => {
                self.with_expected(format!("result[ok]"))
                    .with_actual(format!("result[error]<{:?}>", err))
                    .fail();
            }
        };

        self
    }

    /// Asserts that the subject is `Err`. The value type must be a `Result`.
    ///
    /// ```rust,ignore
    /// assert_that(&Result::Err::<usize, usize>(1)).is_error();
    /// ```
    fn is_error(&mut self) -> &mut Self {
        match self.subject {
            &Err(_) => (),
            &Ok(ref val) => {
                self.with_expected(format!("result[error]"))
                    .with_actual(format!("result[ok]<{:?}>", val))
                    .fail();
            }
        };

        self
    }
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;

    #[test]
    fn should_not_panic_if_result_is_expected_to_be_ok_and_is() {
        let result: Result<&str, &str> = Ok("Hello");
        assert_that(&result).is_ok();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: result[ok]\n\t but was: result[error]<\"Oh no\">")]
    fn should_panic_if_result_is_expected_to_be_ok_and_is_not() {
        let result: Result<&str, &str> = Err("Oh no");
        assert_that(&result).is_ok();
    }

    #[test]
    fn should_not_panic_if_result_is_expected_to_be_error_and_is() {
        let result: Result<&str, &str> = Err("Oh no");
        assert_that(&result).is_error();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: result[error]\n\t but was: result[ok]<\"Hello\">")]
    fn should_panic_if_result_is_expected_to_be_error_and_is_not() {
        let result: Result<&str, &str> = Ok("Hello");
        assert_that(&result).is_error();
    }

}
