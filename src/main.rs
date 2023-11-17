
//Features to add:
//GENERAL MAU-MAU RULES
//- "SEVEN" next Player needs to draw two cards,
//   can be countered by another seven,
//   adds draw amount
//- "EIGHT" next player needs to sit out this turn
//- "JACK (BUBE)" can be laid on any card, choose a symbol
//  JACK on JACK is not allowed
//- two cards in hand,
//  "Mau" needs to be said then laying card, else draw two cards
//- one card in hand,
//  "Mau-Mau" needs to be said...
//
//EXTRA RULES (prob just UNO stuff)
//- Reverse Player order
//-
//
//EXTRA Features
//- graphical ui/2D renderer
//- online functionality
//- multiple game windows for each player




use card_games::{network, session_controller::SessionController, my_json::*};

fn main() {
    tests::test_json_to_string();

    maumau_with_session_control();

   }


fn maumau_with_session_control(){
    let (http_request_send, http_request_rec) = crossbeam_channel::unbounded();
    let (http_response_send, http_response_rec) = crossbeam_channel::unbounded();

    std::thread::spawn(move || {network::main(http_request_send, http_response_rec); });

    let mut session_controller = SessionController::new(http_request_rec, http_response_send);
/*
    let arc = Arc::new(Mutex::new(session_controller));
    SessionController::main(&arc);
*/
    session_controller.run();
}

fn maumau_single_session(){
    /*let (sender, reciever) = crossbeam_channel::unbounded();
    let (httpSender, httpReciever) = crossbeam_channel::unbounded();
    let sender2 = sender.clone();
    std::thread::spawn(move || {network::main(sender2, httpReciever) });

    let mut maumau_game = MauMauCardGame::new();
    maumau_game.setup();
    maumau_game.print_player_hand();
    while let Ok(input) = reciever.recv() {
        if let Ok(input) = input.parse() {
            maumau_game.gameplay_step(input);
            maumau_game.print_player_hand();
            let response = Response::new("HTTP/1.1 200 OK",  maumau_game.player_state_json().into());
            if let Ok(()) = httpSender.send(response){
                println!("maumau response");
            }
        }
        else {
            println!("wrong input");
        }
    }
    println!("cardgame reciever disconnected");*/


}



