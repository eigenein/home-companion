use anyhow::Error;

use crate::logging::error;

pub trait LoggedUnwrap<T> {
    #[deprecated = "introduce a proper serializable error"]
    fn unwrap_logged(self) -> T;
}

impl<T> LoggedUnwrap<T> for Result<T, Error> {
    fn unwrap_logged(self) -> T {
        match self {
            Ok(value) => value,
            Err(inner) => {
                error(&format!("Error: {inner:#}"));
                panic!();
            }
        }
    }
}
