
//example.com/path?param1=val&param2=val

use std::{collections::HashMap, error::Error};

use super::error::RequestParseError;

//pub enum value{

type Query = HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct URL {
    pub path : String,
    pub resource_type : Option<String>,
    pub query : Query,
}

impl URL{
    pub fn new() -> Self{
        Self{
            path: String::from("/"),
            resource_type: None,
            query: HashMap::new(),
        }
    }

    fn to_query(query_str : &str) -> Result<Query, Box<dyn Error>>{
        let mut query : Query = HashMap::new();
        for param in query_str.split("&"){
            if let Some((name, value)) = param.split_once("="){
                query.insert(name.to_string(), value.to_string());
            }
            else{ return Err(Box::new(RequestParseError)); }
        }

        Ok(query)
    }

    fn get_file_type(path : &str) -> Option<String>{
        match path.split_once(".") {
            Some((_file_path, file_type)) => Some(String::from(file_type)),
            None => None,
        }
    }
}

impl TryFrom<&str> for URL{
    type Error = Box<dyn Error>;
    fn try_from(url: &str) -> Result<Self, Self::Error> {
        if let Some((path, query)) = url.split_once("?") {

            let url = URL{
                path : path.to_string(),
                resource_type : Self::get_file_type(path),
                query : Self::to_query(query)?,
            };
            return Ok(url);
        }
        else {
            let url = URL{
                path : url.to_string(),
                resource_type : Self::get_file_type(url),
                query : HashMap::new(),
            };
            return Ok(url);
        }
    }
}
