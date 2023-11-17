//use std::io;
//use std::sync::mpsc;
use rand::Rng;
use crate::card_games::*;
use crate::maumau_cardgame::CardSymbol::*;
use crate::my_json::{JSONObject, Keyvalue};

pub enum PlayerAction {
    LayCard(usize),
    LayCardUncheck(usize),
    SayMauMau(PlInpData, Box<PlayerAction>),
    ColorWish(CardSymbol, Box<PlayerAction>),
    Pass,
}

pub struct PlInpData {
    pub card_input : usize,
    pub mau_count : usize,
    pub symbol_wish : Option<CardSymbol>,
}

type DynCardeffect = Box<dyn CardEffect + Send + Sync>;

pub struct MauMauCardGameState{
    pub deck : Vec<Card>,
    pub layed_cards : Vec<Card>,
    pub players : Vec<CardPlayer>,
    pub player_order : Vec<usize>,
    pub cur_player_index : usize,
}

pub struct MauMauCardGame{
    pub state : MauMauCardGameState,
    pub cur_card_effect : DynCardeffect,
    pub laying_card_effect : DynCardeffect,

    pub _draw_card_amount : u32,
    pub _turn_dir_clockwise : bool,
    pub turn_skip : usize,

}

impl MauMauCardGame{

    /*TODO:
     * - add variable number of players
     */

    pub fn new() -> Self{
        MauMauCardGame{
            state : MauMauCardGameState{
              deck: Vec::with_capacity(36),
              layed_cards: Vec::with_capacity(36),
              players: Vec::new(),
              player_order: Vec::new(),
              cur_player_index : 0,
            },

            cur_card_effect : Box::new(StandardCardEffect{}),
            laying_card_effect : Box::new(StandardCardEffect{}),

            _draw_card_amount : 0,
            _turn_dir_clockwise : false,
            turn_skip : 1,
        }
    }

    pub fn player_state_json(&self, player_id : usize) -> JSONObject {
        let mut pl_json = JSONObject::new();
        let pl_cards = &self.state.players[player_id].hand;
        //TODO: handle unwrap err
        pl_json.add("top_card".into(),
                    self.state.layed_cards.last()
                    .unwrap_or(&Card::new(Heart, 0)).as_json_val());
        pl_json.add("hand_cards".into(),
                    Keyvalue::List(pl_cards.iter().map(
                            |card| {
                                card.as_json_val()
                            }
                            ).collect()
                        )
                    );
        let turn_status = self.get_cur_player_id() == player_id;
        pl_json.add("turn_status".into(),
                    Keyvalue::Value(turn_status.to_string()));

        pl_json.add(
            "opp_card_counts".into(),
            Keyvalue::List(
                self.state.player_order.iter().map(
                    |ind| {
                        let mut obj = JSONObject::new();
                        let pl = &self.state.players[*ind];
                        obj.add("name".into(), Keyvalue::Value(pl.name.clone()));
                        obj.add(
                            "card_count".into(),
                            Keyvalue::Value(pl.hand.len().to_string()));
                        Keyvalue::Object(obj)
                    }
                    ).collect()
                ));

        pl_json
    }

    pub fn setup_deck(&mut self){
        let gamestate = &mut self.state;
    let mut random_deck : Vec<Card> = Vec::with_capacity(36);

    for i in 6..15 {
        /*if i == JACK {
            gamestate.deck.push(Card::new(Heart, i) );
            gamestate.deck.push(Card::new(Heart, i) );
            gamestate.deck.push(Card::new(Heart, i) );
            gamestate.deck.push(Card::new(Heart, i) );
        }
        else{*/
            gamestate.deck.push(Card::new(Heart, i));
            gamestate.deck.push(Card::new(Diamond, i));
            gamestate.deck.push(Card::new(Cross, i));
            gamestate.deck.push(Card::new(Leaf, i));
        //}
    }

    for _i in 0..36 {
        let random_index = rand::thread_rng().gen_range(0..gamestate.deck.len());
        random_deck.push(gamestate.deck.remove(random_index));
    }
    gamestate.deck.append(&mut random_deck);

    gamestate.layed_cards.push(gamestate.deck.pop().unwrap_or(Card::new(Heart, 1)));

    }

    pub fn rnd_player_order(&mut self){
        let player_order = &mut self.state.player_order;
        let length = player_order.len();
        let mut random_order : Vec<usize> = Vec::with_capacity(length);
        for _i in 0..length {
            let random_index = rand::thread_rng().gen_range(0..player_order.len());
            random_order.push(player_order.remove(random_index));
        }
        self.state.player_order = random_order;
    }

    pub fn get_cur_player_mut(&mut self) -> &mut CardPlayer{
        let cur_pl_id = self.state.cur_player_index;
        &mut self.state.players[self.state.player_order[cur_pl_id]]
    }

    pub fn get_cur_player(&mut self) -> &CardPlayer{
        self.get_cur_player_mut()
    }

    pub fn get_cur_player_id(&self) -> usize{
        self.state.player_order[self.state.cur_player_index]
    }


    pub fn add_player(&mut self, name : &str) -> usize{
        let player = CardPlayer::new(name.to_string());
        let players = &mut self.state.players;
        players.push(player);
        let player_id : usize = players.len()-1;
        self.state.player_order.push(player_id);
        player_id
    }


    pub fn add_four_players(&mut self){
         self.state.players.append(&mut vec![
        CardPlayer::new("P1".to_string()),
        CardPlayer::new("P2".to_string()),
        CardPlayer::new("P3".to_string()),
        CardPlayer::new("P4".to_string()),
        ]);
    }

    pub fn players_take_cards(&mut self){
        println!("Players, take your Cards!!!");

         //maybe switch loops around
    for p in self.state.players.iter_mut() {
        for _i in 1..=3 {
            p.hand.push(self.state.deck.pop().unwrap_or(Card::new(Heart, 1)));
        }
    }

    //TODO: error handling

    let laying_card = self.state.layed_cards.last().unwrap();
    if laying_card.value == JACK {
                  self.laying_card_effect = Box::new(
                      JackCardEffect{wanted_symbol: laying_card.symbol.clone()}
                      );
              }
              else {
                  self.laying_card_effect = Box::new(StandardCardEffect{});
              }
    }




    pub fn gameplay_step(&mut self, input : PlayerAction){
        self.turn_skip = 1;

        self.handle_player_action(input);

        self.next_turn(self.turn_skip);
    }

    pub fn print_player_hand(&self){
        let gamestate = &self.state;
        println!("Top Card: {}",
                 gamestate.layed_cards.last().unwrap_or(&Card::new(Heart, 1)));
        println!("{}:", gamestate.players[gamestate.cur_player_index].name);

        let mut player_hand_string = String::new();
        for card in gamestate.players[gamestate.cur_player_index].hand.iter() {
            player_hand_string.push_str(&format!("{}", &card));
        }
        println!("Your Hand: {}", player_hand_string);

    }

    fn next_turn(&mut self, mut add : usize){
        //TODO: mit modulo ersetzen -> unbegrenzter turnskip
        if add > self.state.players.len() { add = self.state.players.len()-1;}
         self.state.cur_player_index += add;
        if self.state.cur_player_index >= self.state.players.len() {
            self.state.cur_player_index -= self.state.players.len();
        }
    }

    fn handle_player_action(&mut self, input : PlayerAction){
        //lay down card from hand
        use PlayerAction::*;
        match input{
            LayCard(card_input) => {
//                let card_count = self.get_cur_player_mut().hand.len();
                //exception for maumau range last two card range
                if self.get_cur_player().hand.len() > 2{
                    self.cardinput(card_input);
                }
                else { self.pass_draw();}
            },
            LayCardUncheck(card_input) => {
                self.cardinput(card_input);
            },
            ColorWish(color_wish, pl_act) => {
                self.cur_card_effect = Box::new(JackCardEffect{wanted_symbol: color_wish});
                self.handle_player_action(*pl_act);
            },
            SayMauMau(inp, pl_act) => {
                let card_count = self.get_cur_player_mut().hand.len();
                //mau * 2 == cards 1;; mau * 1 == cards 2
                if inp.mau_count <= 2 &&
                    ((inp.mau_count == 1 && card_count == 2)
                    ||
                    (inp.mau_count == 2 && card_count == 1)){
                    self.handle_player_action(*pl_act);
                    println!("maumaued");
                }
                else if inp.mau_count == 0 && card_count > 2{
                    self.handle_player_action(*pl_act);
                }
                else{
                    self.pass_draw();
                    println!("maumaues at wrong time");
                }

            },
            Pass => {self.pass_draw();},
        };

    }

    fn cardinput(&mut self, input : usize){
        //let gamestate = &mut self.state;
        if input <  self.get_cur_player_mut().hand.len() {


            //set cardeffects before inputcardvalidation
              let choosen_hand_card : &Card =
                  &self.get_cur_player_mut().hand[input as usize];
              if choosen_hand_card.value == JACK {
                  //self.cur_card_effect = Box::new(JackCardEffect{wanted_symbol: choosen_hand_card.symbol.clone()});
              }
              else {
                  self.cur_card_effect = Box::new(StandardCardEffect{});
              }



            //Effect: Pre Input Processing
            self.laying_card_effect.pre_inp_proc_eff(&mut self.state);

            if self.laying_card_effect.laying_cond(&mut self.state, input)
            || self.cur_card_effect.setting_cond(){

              if self.get_cur_player_mut().hand[input as usize].value == SEVEN {}
              else if self.get_cur_player_mut().hand[input as usize].value == EIGHT {
                  self.turn_skip = 2;
                  //self.next_turn(1);
                  println!("{}'s turn was skipped",
                           &self.get_cur_player_mut().name);
              }


              //Effect: Post Input Processing
              self.cur_card_effect.post_inp_proc_eff(&mut self.state);

              let card =
              self.get_cur_player_mut().hand.remove(input as usize);
              self.state.layed_cards.push(card);

              //Effect: Post End GameState
              self.cur_card_effect.post_end_gamestate_eff(&mut self.state);

              std::mem::swap(&mut self.laying_card_effect, &mut self.cur_card_effect);

            }
            else {
                self.pass_draw();
           }

        }
        //draw card from deck
        else if input >= self.get_cur_player_mut().hand.len() {
            self.pass_draw();
       }
    }

    fn pass_draw(&mut self){
        let drawn_card = match self.state.deck.pop(){
            Some(card) => card,
            None => {
                self.shuffle_layed_card_back();
                self.state.deck.pop().unwrap_or(Card::new(Heart, 1))
            },
        };

            self.get_cur_player_mut().hand.push(drawn_card);
    }

    fn shuffle_layed_card_back(&mut self){
        let gamestate = &mut self.state;
        let num_of_rnd_gen = gamestate.layed_cards.len();
    for _i in 0..num_of_rnd_gen {
        let random_index = rand::thread_rng().gen_range(0..gamestate.layed_cards.len());
        gamestate.deck.push(gamestate.layed_cards.remove(random_index));
    }

    gamestate.layed_cards.push(gamestate.deck.pop().unwrap_or(Card::new(Heart, 1)));
    }

}


pub struct StandardCardEffect {
}
unsafe impl Send for StandardCardEffect{}
unsafe impl Sync for StandardCardEffect{}
impl CardEffect for StandardCardEffect{
    fn laying_cond(&self, card_game : &mut MauMauCardGameState, hand_card_index : usize) -> bool {
        let choosen_hand_card = &card_game
           .players[card_game.player_order[card_game.cur_player_index]]
           .hand[hand_card_index];

        let symbol = Card::new(Heart, 1);
        let top_card = &card_game.layed_cards.last()
            .unwrap_or(&symbol);

        choosen_hand_card.value == top_card.value
        || choosen_hand_card.symbol == top_card.symbol
    }
}


#[derive(Debug)]
pub struct JackCardEffect {
   pub wanted_symbol : CardSymbol,
}
unsafe impl Send for JackCardEffect{}
unsafe impl Sync for JackCardEffect {}
impl CardEffect for JackCardEffect {
    fn laying_cond(&self, card_game : &mut MauMauCardGameState, hand_card_index : usize) -> bool {
        let choosen_hand_card = &card_game
           .players[card_game.player_order[card_game.cur_player_index]]
           .hand[hand_card_index];

        //choosen_hand_card.value == card_game.layed_cards.last().unwrap().value
        //||
            self.wanted_symbol == choosen_hand_card.symbol
    }

    fn setting_cond(&self) -> bool {
        true
    }

    fn post_inp_proc_eff(&mut self, _card_game : &mut MauMauCardGameState) {
        println!("JACK goes with everything");
        println!("Choose a Symbol: 1:Heart, 2:Diamond, 3:Cross, 4:Leaf");
/*        let mut text_input = String::new();
        let mut input : i32 = -1;
                  io::stdin().read_line(&mut text_input).expect("wrong, idiot");
                  if let Ok(num_input) = text_input.trim().parse() {
                    input = num_input;
                  }

                  match input {
                      1 => self.wanted_symbol = Heart,
                      2 => self.wanted_symbol = Diamond,
                      3 => self.wanted_symbol = Cross,
                      4 => self.wanted_symbol = Leaf,
                      _ => {},
                  }
                  println!("You chose {}!", self.wanted_symbol);*/
              }
}

