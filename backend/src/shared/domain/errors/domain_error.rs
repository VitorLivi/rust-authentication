use std::fmt;


#[derive(Debug)]
pub enum DomainError {
    TestError,
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
