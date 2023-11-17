use std::collections::HashMap;


use crate::maumau_cardgame::{self, MauMauCardGame};

use super::{method::Method, request::Request, response::Response};

// Generic Parameter R
// is meant to be the Resourcetype that is passed to the handler functions

pub type HandlerFunc<R> = Box<dyn FnMut(&mut R, &Request) -> Response + Send>;
type HandlerFuncMap<R> = HashMap<String, HandlerFunc<R>>;


pub struct RequestHandler<R>{
    pub method_map :
        HashMap<Method, HandlerFuncMap<R>>,
}

//unsafe impl<R> Send for RequestHandler<R>{}

impl<R> RequestHandler<R>{
    pub fn new() -> Self{
        RequestHandler{
            method_map : HashMap::new()
        }
    }

    pub fn print(&self){
        for (k, v) in self.method_map.iter(){
        println!("key: {:?}, val", k);
        }
    }

    pub fn add_handler(
        &mut self,
        method : &Method,
        path : &str,
        func : HandlerFunc<R>
        ){
        match self.method_map.get_mut(method){
            Some(func_map) => {
                func_map.insert(path.into(), func);
            },
            None =>{
                let mut func_map : HandlerFuncMap<R> = HashMap::new();
                func_map.insert(path.into(), func);
                self.method_map.insert(method.clone(), func_map);
            },
        };
    }

    pub fn clear(&mut self){
        self.method_map = HashMap::new();
    }

    pub fn handle_request(&mut self, resource : &mut R, request : &Request) -> Result<Response, String>{
        if let Some(func_map) = self.method_map.get_mut(&request.method){
            if let Some(func) = func_map.get_mut(&request.url.path){
                Ok(func(resource, request))
            }
            else{
                self.print();

                Err("HTTP/1.1 400 NO PATH FOUND".into())
            }
        }
        else {
            self.print();
                Err("HTTP/1.1 400 METHOD NOT SUPPORTED".into())
        }

    }
}


fn handler_function(game: &mut MauMauCardGame, request : &Request) -> Response{
    println!("hello handler");
    Response::new("HTTP/1.1 200 OK".into(), "balablab".into())
}

fn handler_function2(game: &mut MauMauCardGame, request : &Request) -> Response{
    println!("hello handler");
    Response::new("HTTP/1.1 200 OK".into(), "balablab".into())
}

fn test(){
    let mut game: MauMauCardGame = MauMauCardGame::new();
    game.setup_deck();
    let req = Request::new();
    let mut handler : RequestHandler<MauMauCardGame>= RequestHandler{
        method_map : HashMap::new(),
    };

    let mut map : HandlerFuncMap<MauMauCardGame> = HashMap::new();
    map.insert("test".into(), Box::new(handler_function));

    handler.method_map.insert(Method::GET, HashMap::new());

    handler.add_handler(&Method::GET, "er/sa/de".into(), Box::new(handler_function));
    //handler.method_map.get(&Method::GET).unwrap().get("test").unwrap()();
    handler.handle_request(&mut game, &req);
}

