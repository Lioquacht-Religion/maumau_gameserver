use std::{io::{BufReader, BufRead}, net::TcpStream, error::Error};
use super::{method::Method, error::RequestParseError, url::URL, header::HeaderSet};

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub url : URL,
    pub version : String,
    pub headers : HeaderSet,
    pub body : Vec<u8>,
}

impl Request{
    pub fn new() -> Self{
        Self{
            method: Method::GET,
            url: URL::new(),
            version: String::new(),
            headers: HeaderSet::new(),
            body: Vec::new(),
        }
    }

    pub fn build(buf_reader : &mut BufReader<&mut TcpStream>) -> Result<Self, Box<dyn Error>> {
        let mut request = Request::new();
        request.read_stanza(buf_reader)?;
        request.read_header(buf_reader)?;
        if request.has_body(){
            request.read_body(buf_reader)?;
        }
        Ok(request)
    }

    pub fn has_body(&self) -> bool {
        self.method == Method::POST
    }

    pub fn read_stanza(&mut self, buf_reader : &mut BufReader<&mut TcpStream>)
    -> Result<(), Box<dyn Error>>{
        if let Some(line) = buf_reader.lines().next() {
            let line = line.unwrap_or(String::new());
            let req_line : Vec<&str> = line.split_whitespace().collect();
            if req_line.len() < 3{
                return Err(Box::new(RequestParseError));
            }
            self.method = Method::try_from(req_line[0])?;
            self.url = URL::try_from(req_line[1])?;
            self.version = req_line[2].to_string();
        }

        return Ok(());
    }

    pub fn read_header(&mut self, buf_reader : &mut BufReader<&mut TcpStream>)
    -> Result<(), Box<dyn Error>>{
        self.headers = HeaderSet::try_from(buf_reader)?;
        Ok(())
    }

    pub fn read_body(&mut self, buf_reader : &mut BufReader<&mut TcpStream>)
    -> Result<(), Box<dyn Error>>{
        //need to search self for content-length header, to not search infinitly
        self.body = buf_reader.buffer().iter().map(|b| *b).collect();
        Ok(())
        /*let buffer = buf_reader.buffer();
        let mut byte_pos : usize = 0;
        println!("buffer: {}", String::from_utf8_lossy(buffer));
        for line in buffer.lines(){
            match line {
                Ok(line) => {
                   byte_pos += line.as_bytes().len();
                   if line.is_empty(){
                       let (_head_section, body_section) = buffer.split_at(byte_pos);
                       println!("body: {}", String::from_utf8_lossy(body_section));
                       self.body = body_section.iter().map(|b| *b).collect();
                       return Ok(());
                   }
                },
                Err(e) => return Err(Box::new(e)),
            };

        }
        println!("nothing to read");
        Err(Box::new(RequestParseError))*/
    }

}
