use std::{fmt::Display, io::{BufReader, BufRead}, net::TcpStream};


#[derive(Debug)]
pub enum HttpKind {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl Display for HttpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<&str> for HttpKind{
    type Error = HTTPParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(HttpKind::GET),
            "POST" => Ok(HttpKind::POST),
            "PUT" => Ok(HttpKind::PUT),
            "PATCH" => Ok(HttpKind::PATCH),
            "DELETE" => Ok(HttpKind::DELETE),
            _ => Err(HTTPParseError),

        }

    }
}


//HTTP Request:
//Method Request-URI HTTP-Version CRLF
//headers CRLF
//message-body

//Response:
//HTTP-Version Status-Code Reason-Phrase CRLF
//headers CRLF

pub struct HTTPParseError;

#[derive(Debug)]
pub struct HTTPRequest{
    pub kind : HttpKind,
    pub uri : String,
    pub version: String,
    pub headers: Vec<String>,
    pub body: Vec<u8>,
}

impl HTTPRequest{
    pub fn new() -> Self{
        HTTPRequest{
            kind : HttpKind::GET,
            uri : String::from("/"),
            version : String::from("/"),
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    fn reqline_from_string(mut self, req_base_info : String) -> Result<Self, HTTPParseError>{
        let req_base_info : Vec<&str> = req_base_info.split_whitespace().collect();
        if req_base_info.len() < 3{
            println!("missing method, uri or version!");
            return Err(HTTPParseError);
        }

        match HttpKind::try_from(req_base_info[0]) {
            Ok(kind) => self.kind = kind,
            Err(e) => return Err(e),
        }

        self.uri = req_base_info[1].to_string();
        self.version = req_base_info[2].to_string();
        return Ok(self);
    }
}

/*impl TryFrom<&String> for HTTPRequest{
    type Error = HTTPParseError;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        HTTPRequest::try_from(value.split("\r\n").collect::<Vec<String>>())
    }
}*/

impl TryFrom<BufReader<&mut TcpStream>> for HTTPRequest{
    type Error = HTTPParseError;
    fn try_from(mut buf_reader: BufReader<&mut TcpStream>) -> Result<Self, Self::Error> {
        //let mut buf_reader = buf_reader.lines();
        let mut request = HTTPRequest::new();
        /*match buf_reader.next() {
            Some(req_line) => {
                if let Ok(req) = request.reqline_from_string(req_line.unwrap_or("".to_string())){
                    request = req;
                }
                else{ return Err(HTTPParseError);}
            },
            None => { return Err(HTTPParseError);}
        };*/

        /*for line_crlf in buf_reader{
            if let Ok(line_crlf) = line_crlf {
                if !line_crlf.is_empty() {
                    request.headers.push(line_crlf);
                }
                else {
                    break;
                }
            }
        }*/

        let mut req_line = String::new();
        buf_reader.  read_line(&mut req_line);

        if let Ok(req) = request.reqline_from_string(req_line){
            request = req;
        }
        else{
            return Err(HTTPParseError);
        }



        /*loop {
            println!("looping");
            if let Some(line_crlf) = buf_reader.next(){

                match line_crlf{
                    Ok(line_crlf) => {
                        if !line_crlf.is_empty() {
                            request.headers.push(line_crlf);
                        }
                        else{
                            buf_reader.
                            /*if let Some(body) = buf_reader.next(){
                                if let Ok(body) = body {
                                    println!("making body: {}", &request.body);
                                    request.body = body;
                                    return Ok(request);
                                }
                                else{
                                    return Err(HTTPParseError);
                                }
                            }*/
                            break;
                        }
                    },
                    Err(_e) => {return Err(HTTPParseError);},
                }
            }
            else {
                break;
            }
        }
        println!("now returning");*/


        return Ok(request);

    }
}

impl TryFrom<Vec<String>> for HTTPRequest{
    type Error = HTTPParseError;
    fn try_from(req_sep_crlf: Vec<String>) -> Result<Self, Self::Error> {
        let mut request = Self::new();
        if req_sep_crlf.len() < 1{
            return Err(HTTPParseError);
        }

        for line_crlf in req_sep_crlf.iter(){
            if !line_crlf.is_empty(){
                request.headers.push(line_crlf.to_string());
            }
            else {
                request.body = line_crlf.to_string();
            }
        }

        return request.reqline_from_string(req_sep_crlf[0].to_string());
    }
}


pub struct HTTPResponse;


