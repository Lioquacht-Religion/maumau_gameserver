
#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub enum Method{
    GET,
    POST,
    PUT,
    DELETE,
}

use std::error::Error;
use Method::*;
use super::error::RequestParseError;

impl Method{
    pub fn as_string(&self) -> String{
        match self {
            GET => String::from("GET"),
            POST => String::from("POST"),
            PUT => String::from("PUT"),
            DELETE => String::from("DELETE"),
        }
    }
}

impl TryFrom<&str> for Method {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value{
            "GET" => Ok(GET),
            "POST" => Ok(POST),
            "PUT" => Ok(PUT),
            "DELETE" => Ok(DELETE),
            _ => Err(Box::new(RequestParseError)),
        }
    }
}



