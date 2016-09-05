use super::{build_expectation_string, Spec};

use std::fmt::Debug;

pub trait ResultSpec<T, E>
    where T: Debug,
          E: Debug
{
    fn is_ok(&self);
    fn is_error(&self);
}

impl<'s, T, E> ResultSpec<T, E> for Spec<'s, Result<T, E>>
    where T: Debug,
          E: Debug
{
    fn is_ok(&self) {
        match self.subject {
            &Ok(_) => (),
            &Err(ref err) => {
                panic!(build_expectation_string(&"result[ok]",
                                                &format!("result[error]<{:?}>", err)));
            }
        };
    }

    fn is_error(&self) {
        match self.subject {
            &Err(_) => (),
            &Ok(ref val) => {
                panic!(build_expectation_string(&"result[error]",
                                                &format!("result[ok]<{:?}>", val)));
            }
        };
    }
}
