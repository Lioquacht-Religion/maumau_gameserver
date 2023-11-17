use core::fmt::{Display, self};
use std::io;
use CardSymbol::*;

use crate::{maumau_cardgame::MauMauCardGameState, my_json::{JSONObject, Keyvalue}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardSymbol{
    Heart,
    Diamond,
    Cross,
    Leaf,
}

impl Display for CardSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Heart => write!(f, "Heart"),
            Diamond => write!(f, "Diamond"),
            Cross => write!(f, "Cross"),
            Leaf => write!(f, "Leaf"),

        }
    }

}

/*impl From<&str> for CardSymbol {
    fn from(value: &str) -> Self {
        match value {
            "Heart" => Heart,
            "Diamond" => Diamond,
            "Cross" => Cross,
            "Leaf" => Leaf,
            _ => Heart,
        }

    }
}*/

impl TryFrom<&str> for CardSymbol {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Heart" => Ok(Heart),
            "Diamond" => Ok(Diamond),
            "Cross" => Ok(Cross),
            "Leaf" => Ok(Leaf),
            _ => Err(()),
        }
    }
}

//Cardvalue Constants
pub const SIX: u32 = 6;
pub const SEVEN: u32 = 7;
pub const EIGHT: u32 = 8;
pub const NINE: u32 = 9;
pub const TEN: u32 = 10;
pub const JACK: u32 = 11;
pub const QUEEN: u32 = 12;
pub const KING: u32 = 13;
pub const ACE: u32 = 14;


type DynCardEffect = Box<dyn CardEffect + Send + Sync>;

pub struct Card {
    pub symbol: CardSymbol,
    pub value: u32,
    //pub effect: DynCardEffect,
}

impl fmt::Debug for Card{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Card")
            .field("Symbol", &self.symbol)
            .field("Value", &self.value)
            .finish()
    }
}

impl Display for Card{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) ", self.symbol, self.value)
    }
}

impl Card{
    pub fn new(symbol: CardSymbol, value: u32) -> Card{
        Card{ symbol, value,
     //   effect: Box::new(StandardCardEffect{})
        }
    }

    pub fn as_json_val(&self) -> Keyvalue{
        let mut json_obj = JSONObject::new();
        json_obj.add("card".into(), Keyvalue::Value(self.symbol.to_string()));
        json_obj.add("value".into(), Keyvalue::Value(self.value.to_string()));
        Keyvalue::Object(json_obj)
    }
}



pub trait CardEffect{
    fn laying_cond(&self, card_game : &mut MauMauCardGameState, hand_card_index : usize) -> bool {false}
    fn setting_cond(&self) -> bool {false}


    fn pre_inp_proc_eff(&mut self, _card_game : &mut MauMauCardGameState){}
    fn post_inp_proc_eff(&mut self, _card_game : &mut MauMauCardGameState){}
    fn post_end_gamestate_eff(&mut self, _card_game : &mut MauMauCardGameState){}
}


#[derive(Debug)]
pub struct CardPlayer{
   pub name : String,
   pub hand : Vec<Card>,
}

impl CardPlayer {
    pub fn new(name: String) -> CardPlayer {
        CardPlayer{name, hand : vec![]}
    }
}
