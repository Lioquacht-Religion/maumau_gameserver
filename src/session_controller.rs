use std::{collections::HashMap, sync::{Arc, Mutex}, ops::ControlFlow};

use rand::{Rng, distributions::Alphanumeric};

use crate::{maumau_cardgame::{MauMauCardGame, PlayerAction, PlInpData}, my_http::{response::{Response, self}, request::{Request, self}, request_handler::RequestHandler, method::Method}, threadpool, my_json::{JSONObject, Keyvalue}, card_games::{CardPlayer, JACK, CardSymbol}};

/*
 * TODO:
 * API:
 * - create GameSession-
 * - view existing GameSessions
 * - join GameSession BUG: multiple joins possible
 * - start Session
 * - leave session
 * - verification with session key
 * - get current state of own player
 * (Handcards, cur_topdeckcard, players, pl_cardamount
 *  turncount, win/lose state)
 * - send card input
 * - send card input with symbol wish
 * - send mau/maumau
 * -
 *
 * Session features:
 * - Timeout for inactive players -> draw card
 *
 *
 */

struct SessionChannel(crossbeam_channel::Sender<String>, Arc<Mutex<GameSession>>);

type HttpRequestRec = crossbeam_channel::Receiver<(crossbeam_channel::Sender<Response>, Request)>;

pub struct SessionController{
    //pool : threadpool::ThreadPool,
    sessions : HashMap<String, SessionChannel>,
    http_request_rec : HttpRequestRec,
    http_response_send : crossbeam_channel::Sender<Response>,
    req_handler : RequestHandler<SessionController>
}

impl SessionController{

    pub fn run(&mut self){
        let pool = threadpool::ThreadPool::new(8);
        let mut req_h : RequestHandler<SessionController> = RequestHandler::new();
        req_h.add_handler(&Method::GET, "/maumau/session/create", Box::new(Self::create_session));
        req_h.add_handler(&Method::GET, "/maumau/session/all", Box::new(Self::get_all_sessions));

        while let Ok((resp_send, request)) = self.http_request_rec.recv() {
            match req_h.handle_request(self, &request){
                Ok(resp) => {
                if let Ok(()) = resp_send.send(resp){
                     println!("session controller send response");
                }
                },
                Err(mssg) => {println!("{mssg}")},
            }

            let session_key = request.url.query.get("sessionkey")
                .or(request.headers.get("sessionkey"));

            if let Some(session_key) = session_key{
                if let Some(SessionChannel(_resp, session)) = self.sessions.get_mut(session_key){
                    let session_arc = Arc::clone(session);

                    pool.execute(move || {
                        let mut session = session_arc.lock().unwrap();
                        let resp = session.handle_requests(
                            &request);
                        if let Ok(()) = resp_send.send(resp){
                            println!("session send response");
                        }

                    });
                }
            }
        }

        println!("session controller reciever closed");
    }

    fn handle_request(&mut self, request : &Request) -> Response{
           match request.url.path.as_str() {
                "/maumau/session/create" => {
                    return Self::create_session(self, request);
                },
                /*
                "/maumau/session/all" => {
                    return self.get_all_sessions();
                },
                "/maumau/session/join" => {
                    //TODO: add session state check, if still accepting new players
                    if let Some(session_key) = request.url.query.get("sessionkey"){
                        return self.join_session(session_key);
                    }
                },
                "/maumau/session/start" => {
                    return self.start_session(request);
                },

                "/mauamau/handcard" => {
                    return self.handle_card_input(request);
                },
                "/maumau/handcard/saymau" => {
                    return self.handle_mau_input(request);
                },
                "/maumau/handcard/pass" => {
                    return self.handle_pass_draw(request);
                }
                */
                _ => {},
            };
        return Response::new( "HTTP/1.1 404 NOT FOUND", "too bad no api endpoint".into());
    }

    pub fn new(
        http_request_rec: HttpRequestRec,
        http_response_send: crossbeam_channel::Sender<Response>,
        ) -> Self {

        Self{
            //pool : threadpool::ThreadPool::new(8),
            sessions : HashMap::new(),
            http_request_rec,
            http_response_send,
            req_handler : RequestHandler::new(),
        }
    }

    pub fn create_session(ses_ctrl : &mut SessionController, _request : &Request) -> Response{
        let (sender, reciever) = crossbeam_channel::unbounded();
        let mut session_key = SessionController::generate_session_key();
        while let Some(_key) = ses_ctrl.sessions.get(&session_key) {
            session_key = SessionController::generate_session_key();
        }

        let mut game_session =
                GameSession::new(reciever, ses_ctrl.http_response_send.clone());
        let player_key = game_session.add_player();

        let mut content = JSONObject::new();
        content.add("session_key".to_string(), Keyvalue::Value(session_key.clone()));
        content.add("player_key".into(), Keyvalue::Value(player_key.into()));

        ses_ctrl.sessions.insert(
            session_key,
            SessionChannel(
                sender,
                Arc::new(Mutex::new(game_session)),
                )
            );


            return Response::new("HTTP/1.1 200 OK", content.to_string().into());

    }


    pub fn get_all_sessions(ses_ctrl : &mut SessionController, _request : &Request) -> Response{
        /*
         * JSON content
         * current sessions
         * player count
         *  -- todo: player nicknames
         */
        let mut content = JSONObject::new();
        let mut session_list : Vec<Keyvalue> =
            Vec::with_capacity(ses_ctrl.sessions.capacity());

        for (session_key, SessionChannel(_sender, session)) in ses_ctrl.sessions.iter(){
            let mut json_session = JSONObject::new();
            json_session.add("session_key".into(), Keyvalue::Value(session_key.clone()));
            let session = session.lock().unwrap();
            json_session.add("player_count".into(), Keyvalue::Value(session.data.player_keys.len().to_string()));
            session_list.push(Keyvalue::Object(json_session));
        }
        content.add("sessions".into(), Keyvalue::List(session_list));

        return Response::new("HTTP/1.1 200 OK", content.to_string().into());
    }


    fn generate_session_key() -> String{
        //let session_key : u32 = rand::thread_rng().gen();
        //let session_key : Vec<u8> = session_key.to_be_bytes().into();
        //println!("{}", &String::from_utf8_lossy(&session_key));
        let session_key : String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        //println!("{}", &session_key);
        session_key
    }

}

#[derive(Clone)]
pub enum SessionState{
    ChangeState(Box<SessionState>),
    Waiting,
    InPlay,
}

pub struct GameSessionData{
    pub state : SessionState,
    pub player_keys : HashMap<String, usize>,
    pub game : MauMauCardGame,
}

type ReqHType = GameSessionData;//(&'a mut HashMap<String, usize>, &'a mut MauMauCardGame);

pub struct GameSession{
    pub data : GameSessionData,
    //pub player_keys : HashMap<String, usize>,
    //pub game : MauMauCardGame,
    pub input_reciever : crossbeam_channel::Receiver<String>,
    pub http_resp_sender : crossbeam_channel::Sender<Response>,
    pub req_handler : RequestHandler<ReqHType>,
}

impl GameSession{

    fn new(
        input_reciever : crossbeam_channel::Receiver<String>,
        http_resp_sender : crossbeam_channel::Sender<Response>,
        ) -> Self{
        let game = MauMauCardGame::new();
        let mut req_handler : RequestHandler<ReqHType>= RequestHandler::new();
        Self::setup_req_waiting_handler(&mut req_handler);
        Self{
            data : GameSessionData{
               state : SessionState::Waiting,
               player_keys : HashMap::new(),
               game,
            },
            input_reciever,
            http_resp_sender,
            req_handler,
        }

    }

    fn set_handler_funcs(&mut self){
        use SessionState::*;
        if let ChangeState(ref st) = self.data.state{
            //new_state = Some(*st.clone());
            match **st {
                Waiting => {
                    Self::setup_req_waiting_handler(&mut self.req_handler);
                    self.data.state = *st.clone();
                },
                InPlay => {
                    Self::setup_req_inplay_handler(&mut self.req_handler);
                    self.data.state = *st.clone();
                },
                _ => {},
            }
        }
    }

    fn setup_req_waiting_handler(req_handler : &mut RequestHandler<ReqHType>){
        req_handler.add_handler(&Method::POST, "/maumau/handcard", Box::new(Self::handle_card_input));
        req_handler.add_handler(&Method::POST, "/maumau/session/start", Box::new(Self::start_session));
        req_handler.add_handler(&Method::POST, "/maumau/session/join", Box::new(Self::join_session));
    }

    fn setup_req_inplay_handler(req_handler : &mut RequestHandler<ReqHType>){
        req_handler.add_handler(&Method::GET, "/maumau/player/state", Box::new(Self::get_player_state));
        req_handler.add_handler(&Method::POST, "/maumau/handcard", Box::new(Self::handle_card_input));
        req_handler.add_handler(&Method::POST, "/maumau/handcard/pass", Box::new(Self::handle_pass_draw));
    }

    pub fn handle_requests(&mut self, request : &Request) -> Response{
        match self.req_handler.handle_request(&mut self.data, request){
            Ok(resp) => {
                self.set_handler_funcs();
                return resp;
            }
            Err(mssg) => Response::new(mssg.as_str(), "".into()),
        }

    }

    pub fn add_player(&mut self) -> String{
        self.data.add_player()
    }

    fn start_session(session : &mut ReqHType, _request : &Request) -> Response{
        if let SessionState::InPlay = session.state {
            return Response::new("HTTP/1.1 400 Session already started", "".into());
        }
        session.state = SessionState::ChangeState(Box::new(SessionState::InPlay));
                            session.game.setup_deck();
                            session.game.players_take_cards();
                            return Response::new( "HTTP/1.1 200 session start", "".into());
    }

    fn join_session(data : &mut ReqHType, _request : &Request) -> Response{
        if let SessionState::InPlay = data.state {
            return Response::new("HTTP/1.1 400 cant join session; already in play", Vec::new());
        }
                let player_key = data.add_player();
                let mut content = JSONObject::new();
                content.add("player_key".into(), Keyvalue::Value(player_key));

                return Response::new("HTTP/1.1 200 OK", content.to_string().into());
    }

    fn leave_session(){}
    fn player_give_up(){}

    fn get_player_state(session : &mut ReqHType, request : &Request) -> Response{
     // "/maumau/player/state" => {
                        if let Some(player_key) = request.url.query.get("playerkey"){
                                if let Some(pl_id) = session.player_keys.get(player_key){
                                    return Response::new(
                                        "HTTP/1.1 200 player state send",
                                        session.game.player_state_json(*pl_id).to_string().into()
                                        );

                                }
                            }

                        return Response::new("HTTP/1.1 400 player state not found".into(), vec![]);
    }

    //POST
    fn handle_card_input(session : &mut ReqHType, request : &Request,) -> Response{
        match Self::handle_card_input_response(session, request){
            Some(resp) => resp,
            None => return Response::new("HTTP/1.1 404 JSON File INCORRECT", "".into()),
        }
    }

    fn handle_card_input_response(session : &mut ReqHType, request : &Request) -> Option<Response>{
        //println!("{}", String::from_utf8(request.body.clone())
            //.unwrap_or("".into()).as_str() );
        if let Ok(json) = JSONObject::try_from(
            String::from_utf8(request.body.clone())
            .unwrap_or("".into()).as_str()
            ){

           // println!("json: {}", json.to_string());

            let symbol_wish : Option<CardSymbol>=
            if let Ok(symbol) = CardSymbol::try_from(
                    json.get_ref("symbolwish")?.get_value()?){
                Some(symbol)
            }
            else{ None };

            let pl_inp = PlInpData{
                card_input : json.get_ref("cardinput")?.get_value()?.parse().unwrap_or(100),
                mau_count : json.get_ref("maucount")?.get_value()?.parse().unwrap_or(100),
                symbol_wish,
            };

            use PlayerAction::*;

            let pl_act = if let Some(symbol) = pl_inp.symbol_wish.clone(){
                ColorWish(symbol, Box::new(
                        LayCardUncheck(pl_inp.card_input)
                        ))
            }
            else{
               LayCardUncheck(pl_inp.card_input)
            };

            if let Some((_player_key, player_id)) = session.requester_is_cur_player(request){
                    session.game.print_player_hand();
                    session.game.gameplay_step(
                    PlayerAction::SayMauMau(
                            pl_inp,
                            Box::new(
                                pl_act
                                )
                    ));
                    session.game.print_player_hand();

                       return Some(Response::new( "HTTP/1.1 200 OK",
                                              session.game.player_state_json(
                                                player_id
                                    ).to_string().into()));


            }

            return Some(Response::new("HTTP/1.1 404 NOT YOUR TURN", "".into()));

        }

        return Some(Response::new("HTTP/1.1 404 COULDNT PARSE JSON", "".into()));

    }

    fn handle_pass_draw(session : &mut ReqHType , request : &Request) -> Response{
        if let Some((_player_key, player_id)) = session.requester_is_cur_player(request){
                            session.game.print_player_hand();
                            session.game.gameplay_step(
                                            PlayerAction::Pass
                                            );
                            session.game.print_player_hand();
                            return Response::new( "HTTP/1.1 200 OK",
                                                              session.game.player_state_json(
                                                                  player_id
                                                                  ).to_string().into()
                                                              );

        }
        return Response::new("HTTP/1.1 404 NOT YOUR TURN", "".into());
    }


}

impl GameSessionData{
    pub fn add_player(&mut self) -> String{
        let mut player_key = Self::generate_player_key();
        while let Some(_kv) = self.player_keys.get(&player_key){
            player_key = Self::generate_player_key();
        }
        let player_id = self.game.add_player(&player_key);
        self.player_keys.insert(player_key.clone(), player_id);

        player_key
    }

    pub fn is_cur_player(&self, player_key : &str) -> bool{
        match self.player_keys.get(player_key){
            Some(player_id) => &self.game.get_cur_player_id() == player_id,
            None => false,
        }
    }

    pub fn is_cur_player_id(&self, player_id : usize) -> bool{
        self.game.get_cur_player_id() == player_id
    }

    pub fn get_player_id(&self, player_key : &str) -> Option<&usize>{
        self.player_keys.get(player_key)
    }

    pub fn generate_player_key() -> String {
       let player_key : String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        player_key


    }

    /*pub fn get_player_id(&self, player_key : &str) -> Option<usize>{
        let player = self.player_keys.get(player_key)?;
        play

    }*/

    fn requester_is_cur_player<'a>(&mut self, request : &'a Request) -> Option<(&'a str, usize)> {
        let player_key = request.url.query.get("playerkey").or(request.headers.get("playerkey"));
                                if let Some(player_key) = player_key{
                                    if let Some(player_id) = self.player_keys.get(player_key){
                                        if self.is_cur_player_id(*player_id){
                                            return Some((player_key, *player_id));
                                        }
                                    }
                                }
                 None
    }


}


#[cfg(test)]
mod tests{
    use super::SessionController;

    #[test]
    fn session_key_gen(){
        SessionController::generate_session_key();
    }
}

