use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct RequestParseError;

impl Display for RequestParseError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RequestParseError")
    }
}
impl Error for RequestParseError{
}
