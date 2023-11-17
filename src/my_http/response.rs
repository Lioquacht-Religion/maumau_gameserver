use super::header::HeaderSet;

#[derive(Debug)]
pub struct Response{
    pub status_line : String,
    pub headers : HeaderSet,
    pub content : Vec<u8>,
}

impl Response{


    pub fn new(status_line : &str, content : Vec<u8>) -> Self{
        let mut headers = HeaderSet::new();
        headers.add("Content-Length", &format!("{}", content.len()));
        Self{
            status_line : status_line.to_string(),
            headers,
            content,
        }
    }

    pub fn as_string(&self) -> String{
        format!(
            "{}\r\n{}\r\n{}",
            self.status_line, self.headers.as_string(), String::from_utf8_lossy(&self.content)
        )
    }

    pub fn as_bytes(&self) -> Vec<u8>{
        let mut resp_bytes = Vec::new();
        let l_temp = format!("{}\r\n{}\r\n", self.status_line, self.headers.as_string());
        for b in l_temp.as_bytes().iter(){
            resp_bytes.push(*b);
        }
        for b in self.content.iter(){
            resp_bytes.push(*b);
        }

        resp_bytes
    }
}
