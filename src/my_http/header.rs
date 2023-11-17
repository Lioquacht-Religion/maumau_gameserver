use std::{collections::HashMap, error::Error, io::{BufReader, BufRead}, net::TcpStream};

use super::error::RequestParseError;

#[derive(Debug, Clone)]
pub struct HeaderSet{
    headers : HashMap<String, String>,
}

impl HeaderSet{
    pub fn new() -> Self{
        Self{
            headers : HashMap::new(),
        }
    }

    pub fn add(&mut self, name : &str, value : &str){
        self.headers.insert(String::from(name), String::from(value));
    }

    pub fn get(&self, header : &str) -> Option<&String>{
        self.headers.get(header)
    }

    pub fn get_mut(&mut self, header : &str) -> Option<&mut String>{
        self.headers.get_mut(header)
    }


    pub fn as_string(&self) -> String{
        let mut ret_string = String::new();
        /*self.headers.iter().map(|(name, value)| {
            ret_string.push_str(&format!("{}: {}\r\n", name, value));
        }).collect();*/

        for (name, value) in self.headers.iter(){
             ret_string.push_str(&format!("{}: {}\r\n", name, value));
        }

        ret_string
    }

}

impl TryFrom<&mut BufReader<&mut TcpStream>> for HeaderSet{
    type Error = Box<dyn Error>;
    fn try_from(buf_reader: &mut BufReader<&mut TcpStream>) -> Result<Self, Self::Error> {
        let mut headers : HashMap<String, String> = HashMap::new();
        for line in buf_reader.lines(){
            match line {
                Ok(line) => {
                    if line.is_empty(){
                        return Ok(Self { headers });
                    }
                    if let Some((name, value)) = line.split_once(": "){
                        headers.insert(name.to_string(), value.to_string());
                    }
                    else{
                        return Err(Box::new(RequestParseError));
                    }
                },
                Err(_e) => return Err(Box::new(_e)),
            };
        }
        Ok(Self { headers })
    }
}
