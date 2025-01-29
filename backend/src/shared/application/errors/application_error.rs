use std::fmt;

#[derive(Debug)]
pub enum ApplicationError {
    TestError,
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
