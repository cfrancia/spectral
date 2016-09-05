use super::Spec;

use std::fmt::Debug;

impl<'s, T, E> Spec<'s, Result<T, E>>
    where T: Debug,
          E: Debug
{
    pub fn is_ok(&self) {
        match self.subject {
            &Ok(_) => (),
            &Err(ref err) => {
                panic!(format!("expected ok result but was error result of <{:?}>", err))
            }
        };
    }

    pub fn is_error(&self) {
        match self.subject {
            &Err(_) => (),
            &Ok(ref val) => {
                panic!(format!("expected error result but was ok result of <{:?}>", val))
            }
        };
    }
}
