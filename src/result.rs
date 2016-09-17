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
