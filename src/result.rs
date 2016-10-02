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
