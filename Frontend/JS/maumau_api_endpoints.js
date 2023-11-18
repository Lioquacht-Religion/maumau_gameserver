

function getGameState(){
const session_key = localStorage.getItem("session_key");
const player_key = localStorage.getItem("player_key");
const url = base_api + "/maumau/player/state?sessionkey=" + session_key
        + "&playerkey=" + player_key;

fetch(url, {
  method: "GET",
  headers: {
    "Content-type": "application/json; charset=UTF-8"
  }
})
  .then(response => response.json())
    .then(json => {
        console.log(json);
        updatePlayerState(json);
    });

}

function sendCardInput(cardbutton, input){

if (cardbutton.dataset.cardvalue == 11) {
    let symbolsect = document.getElementById("ChooseSymbolSection");
    symbolsect.hidden = false;
    symbolsect.dataset.index = input;
    document.getElementById("PlaySection").hidden = true;
    return;
}

const session_key = localStorage.getItem("session_key");
const player_key = localStorage.getItem("player_key");
const body = {
    //"playerkey" : player_key,
    "cardinput" : ""+input,
    "maucount" : ""+window.mau_counter,
    "symbolwish" : ""+null
};
let url = base_api + "/maumau/handcard?sessionkey=" + session_key
        + "&playerkey=" + player_key;

fetch(url, {
  method: "POST",
  headers: {
    "Content-type": "application/json; charset=UTF-8",
    "sessionkey" : session_key,
    "playerkey" : player_key
  },
  body : JSON.stringify(body)
})
  .then(response => response.json())
    .then(json => {
        console.log(json);
        updatePlayerState(json);
    });

}

function sendWishCardInput(cardbutton){

const session_key = localStorage.getItem("session_key");
const player_key = localStorage.getItem("player_key");
const input = document.getElementById("ChooseSymbolSection").dataset.index;
const body = {
    "cardinput" : ""+input,
    "maucount" : ""+window.mau_counter,
    "symbolwish" : ""+cardbutton.value
};

const url = base_api + "/maumau/handcard?sessionkey=" + session_key
        + "&playerkey=" + player_key;

fetch(url, {
  method: "POST",
  headers: {
    "Content-type": "application/json; charset=UTF-8",
    "sessionkey": session_key,
      "playerkey": player_key
  },
  body: JSON.stringify(body)
})
  .then(response => response.json())
    .then(json => {
        console.log(json);
        updatePlayerState(json);
    });

document.getElementById("PlaySection").hidden = false;
document.getElementById("ChooseSymbolSection").hidden = true;

}

function getCardElem(element){
     let btn = document.createElement("div");
        btn.className = "card";
        btn.dataset.cardvalue = element.value;
        //btn.innerText = "" + index + " : " + element.card + " " + element.value;

        let symbol = document.createElement("h1");
        symbol.className = "cardsymbol";
        symbol.innerText = element.card;
        let cardval = document.createElement("h1");
        cardval.className = "cardnumber";
        cardval.id = "topnumber";
        cardval.innerText = element.value;
        let cardval2 = document.createElement("h1");
        cardval2.className = "cardnumber";
        cardval2.id = "bottomnumber";
        cardval2.innerText = element.value;
        btn.appendChild(symbol);
        btn.appendChild(cardval);
        btn.appendChild(cardval2);

    return btn;
}

function updatePlayerState(state){
    const TopcardWrapper = document.getElementById("topcard");
    TopcardWrapper.innerHTML = "";
    const HCardWrapper = document.getElementById("PlayerHCards");
    HCardWrapper.innerHTML = "";

    let h1_topcard = document.createElement("h1");
    h1_topcard.innerText = "Topcard: " + state.top_card.card
        + " " + state.top_card.value;
    TopcardWrapper.appendChild(h1_topcard);
    TopcardWrapper.appendChild(getCardElem(state.top_card));

    let items = state.hand_cards;

            /*<div class="card">
                <h1 class="cardsymbol">Symbol</>
                <h1 class="cardnumber" id="topnumber">4</h1>
                <h1 class="cardnumber" id="bottomnumber">4</h1>
            </div>*/


    for (let index = 0; index < items.length; index++) {
        const element = items[index];
        const card_elem = getCardElem(element);
        card_elem.value = index;
        card_elem.onclick = function() {sendCardInput(card_elem, index);};
        HCardWrapper.appendChild(card_elem);
        //<button onclick="sendCardInput()" >send handcardnumber</button><br>

    }

    const TurnStateWrapper = document.getElementById("Turnstatus");
    TurnStateWrapper.innerHTML = "";
    let h1_turnstatus = document.createElement("h1");
    h1_turnstatus.innerHTML = "Turnstatus: " + state.turn_status;
    TurnStateWrapper.appendChild(h1_turnstatus);

    const OppCardsWrapper = document.getElementById("OppPlayers");
    OppCardsWrapper.innerHTML = "";

    let players = state.opp_card_counts;

    for (let index = 0; index < players.length; index++) {
        const element = players[index];
        let paragr = document.createElement("p");
        paragr.innerText = "" + element.name + " : " + element.card_count;
        OppCardsWrapper.appendChild(paragr);
    }
}

