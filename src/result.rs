use super::{build_expectation_string, Spec};

use std::fmt::Debug;

impl<'s, T, E> Spec<'s, Result<T, E>>
    where T: Debug,
          E: Debug
{
    pub fn is_ok(&self) {
        match self.subject {
            &Ok(_) => (),
            &Err(ref err) => {
                panic!(build_expectation_string(&"result[ok]",
                                                &format!("result[error]<{:?}>", err)));
            }
        };
    }

    pub fn is_error(&self) {
        match self.subject {
            &Err(_) => (),
            &Ok(ref val) => {
                panic!(build_expectation_string(&"result[error]",
                                                &format!("result[ok]<{:?}>", val)));
            }
        };
    }
}
