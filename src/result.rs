use super::Spec;

use std::fmt::Debug;

pub trait ResultSpec<T, E>
    where T: Debug,
          E: Debug
{
    fn is_ok(self);
    fn is_error(self);
}

impl<'s, T, E> ResultSpec<T, E> for Spec<'s, Result<T, E>>
    where T: Debug,
          E: Debug
{
    fn is_ok(self) {
        match self.subject {
            &Ok(_) => (),
            &Err(ref err) => {
                self.with_expected(format!("result[ok]"))
                    .with_actual(format!("result[error]<{:?}>", err))
                    .fail();
            }
        };
    }

    fn is_error(self) {
        match self.subject {
            &Err(_) => (),
            &Ok(ref val) => {
                self.with_expected(format!("result[error]"))
                    .with_actual(format!("result[ok]<{:?}>", val))
                    .fail();
            }
        };
    }
}
