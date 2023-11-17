use std::{net::{TcpListener, TcpStream}, thread::{self, JoinHandle},
    fs, io::{self, BufReader, BufRead, Write}, process::exit, sync::{mpsc::{channel, Sender, self}, Mutex, Arc}};
use crate::{threadpool, my_http::{self, request::Request, response::Response, method::Method}};

//HTTP Request:
//Method Request-URI HTTP-Version CRLF
//headers CRLF
//message-body

//Response:
//HTTP-Version Status-Code Reason-Phrase CRLF
//headers CRLF



/*fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}*/

type InputSend = crossbeam_channel::Sender<(crossbeam_channel::Sender<Response>, Request)>;
type ResponseRec = crossbeam_channel::Receiver<Response>;

fn start_command_inputs(sender : Sender<i32>) -> thread::JoinHandle<io::Result<()>>{
    thread::spawn(move || -> io::Result<()> {loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
            //println!("your input?: {}", &input);
        let mut cmd_num_inp : i32 = 0;
            if let Ok(num_inp) = input.trim().parse::<i32>() {
                cmd_num_inp = num_inp;
            }

            match cmd_num_inp {
                -1 => {
                   println!("now exiting program");
                   exit(0);
                }
                0 => {},
                _ => {},
            }

            sender.send(cmd_num_inp).unwrap();


    }})
}

fn handle_connection(mut stream: TcpStream, sender : InputSend, reciever : ResponseRec){
    //println!("{}", "\r".as_bytes()[0]);
    let mut buf_reader = BufReader::new(&mut stream);
    let http_req = Request::build(&mut buf_reader);

    let mut request_line = String::new();

    match http_req{
        Ok(http_req) => {
            //dbg!(&http_req);
            let response = handle_request(&http_req, sender, reciever);
            println!("response: {}", &response.headers.as_string());

            match stream.write_all(&response.as_bytes()){
                Ok(()) => {},
                Err(e) => {println!("TCP Stream failed: {}", e)},
            }

            request_line =
                format!("{} {} {}", http_req.method.as_string(), http_req.url.path, http_req.version);
        },
        Err(_e) => {println!("parsing failed");},

    }

    println!("client request: {}", request_line);


    /*
     * resonse type
     * requested file type
     * setting content-headers accordingly
     * -> text/html, text/css, video/mp4, image/png
     * */

    //let response = handle_request(&http_req);

    //println!("response: {}", &response);

}

fn handle_request(request : &Request, sender : InputSend, reciever : ResponseRec) -> Response{
    match request.method {
        Method::GET => {handle_get_request(request, sender, reciever)},
        Method::POST => {handle_get_request(request, sender, reciever)},
        _ => {handle_get_request(request, sender, reciever)},
    }

}

fn handle_get_request(request : &Request, sender : InputSend, reciever : ResponseRec) -> Response{
    if let Some(res_type) = &request.url.resource_type {
        //let res_type :&str = res_type.as_str();
        let file_path = format!("Frontend{}", &request.url.path);
        match res_type.as_str() {
            "html" | "css" | "js" => {
                return Response::new("HTTP/1.1 200 OK", get_file_contents(&file_path).into_bytes());
            },
            "mp4" => {
                let mut resp = Response::new("HTTP/1.1 200 OK", get_file_contents_bytes(&file_path));
                resp.headers.add("Content-Type", "video/mp4");
                return resp;
            }
            "png" => {
                let mut resp = Response::new("HTTP/1.1 200 OK", get_file_contents_bytes(&file_path));
                resp.headers.add("Content-Type", "image/png");
                return resp;
            }
            _ => {
               return Response::new( "HTTP/1.1 200 OK", "".into());
            },
        };
    }
    else{
        let (send, rec) = crossbeam_channel::unbounded();
        if let Ok(()) = sender.send((send, request.clone())){
            println!("http request send to session controller");
            if let Ok(resp) = rec.recv() {
                return resp;
            }
        }

        /*match request.url.path.as_str() {
            "/mauamau/handcard" => {
                if let Some((_key, value)) = request.url.query.get_key_value("handcardnum"){
                    sender.send(value.to_owned()).unwrap();
                    match reciever.recv(){
                        Ok(resp) => return resp,
                        Err(_e) => return Response::new( "HTTP/1.1 200 OK", "maumau request processed".into()),
                    }
                }

                return Response::new( "HTTP/1.1 404 NOT FOUND", get_file_contents_bytes("Frontend/html/404.html"));
            },

            "/maumau/session/create" => {


            },

            _ => {return Response::new( "HTTP/1.1 404 NOT FOUND", get_file_contents_bytes("Frontend/html/404.html"));}
        };*/
        return Response::new( "HTTP/1.1 404 NOT FOUND", get_file_contents_bytes("Frontend/html/404.html"));

    }



}


fn get_file_contents(request : &str) -> String {

    match fs::read_to_string(&request) {
        Ok(string)  => string,
        Err(_e) =>  {
            println!("file {} not found, err: {}", &request, _e);
            fs::read_to_string("Frontend/html/404.html").unwrap()
        },
    }
}

fn get_file_contents_bytes(request : &str) -> Vec<u8> {

    match fs::read(&request) {
        Ok(bytes)  => bytes,
        Err(_e) =>  {
            println!("file {} not found, err: {}", &request, _e);
            fs::read("Frontend/html/404.html").unwrap()
        },
    }
}





pub fn main(sender : InputSend, reciever : ResponseRec){
    let pool = threadpool::ThreadPool::new(8);

    //let (sender, reciever) = mpsc::channel();
    //let mut _commands_thread = start_command_inputs(sender.clone());

    let listener = TcpListener::bind(
        "192.168.2.63:7878"
        //"127.0.0.1:7878"
        ).unwrap();

    for stream in listener.incoming(){

        let sender2 = sender.clone();
        let reciever2 = reciever.clone();
        /*match reciever.try_recv().unwrap(){
            0 => {},
            i => {println!("some other cmd: {}",  i);}
        }*/
        let stream = stream.unwrap();

        pool.execute(move || {handle_connection(stream, sender2.clone(), reciever2.clone())});
    }
}

