function onSubmit(form){
    console.log(form.name.value, form.description.value, form.email.value);
    alert(form.name.value, form.description.value, form.email.value);

}

/* Set the width of the sidebar to 250px (show it) */
function openNav() {
      document.getElementById("mySidepanel").style.width = "50%";
    }

    /* Set the width of the sidebar to 0 (hide it) */
    function closeNav() {
      document.getElementById("mySidepanel").style.width = "0";
    }

function sendSuggestion(form) {
//    console.log(form.name.value, form.description.value, form.email.value);
//    alert(form.name.value, form.description.value, form.email.value);
const xhr = new XMLHttpRequest();
const url = "http://127.0.0.1:7878/html/charsug.html";

const sug_data = JSON.stringify({
    name : form.name.value,
    decription: form.description.value,
    email: form.email.value,
});

xhr.open("POST", url);
// Send the proper header information along with the request
xhr.setRequestHeader("Content-Type", "application/json; charset=UTF-8");

xhr.onload = () => {
  // Call a function when the state changes.
  if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
    // Request finished. Do processing here.
    console.log(Http.responseText);
  }
};

//xhr.send(form);
xhr.send(sug_data);
// xhr.send(new Int8Array());
// xhr.send(document);


}

function testPost(){
const url = "http://127.0.0.1:7878/html/charsug.html";
fetch(url, {
  method: "POST",
  body: JSON.stringify({
    userId: 1,
    title: "Fix my bugs",
    completed: false,
  }),
  headers: {
    "Content-type": "application/json; charset=UTF-8"
  }
})
  .then((response) => console.log(response));
      //response.json())
  //.then((json) => console.log(json));
}

window.addEventListener("load", (event) => {
    getGameState();
  //log.textContent += "load\n";
});

const base_api = "http://192.168.2.63:7878"; //"http://127.0.0.1:7878";

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

function sendCardInput(input){

//const input = document.getElementById("handcardinput");
const session_key = localStorage.getItem("session_key");
const player_key = localStorage.getItem("player_key");
const url = base_api + "/maumau/handcard?sessionkey=" + session_key
        + "&playerkey=" + player_key
        + "&handcardnum=" + input;

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

function updatePlayerState(state){
    const TopcardWrapper = document.getElementById("topcard");
    TopcardWrapper.innerHTML = "";
    const HCardWrapper = document.getElementById("PlayerHCards");
    HCardWrapper.innerHTML = "";

    let h1_topcard = document.createElement("h1");
    h1_topcard.innerText = "" + state.top_card.card
        + " " + state.top_card.value;
    TopcardWrapper.appendChild(h1_topcard);


    let items = state.hand_cards;

    for (let index = 0; index < items.length; index++) {
        const element = items[index];
        let btn = document.createElement("button");
        btn.value = index;
        btn.innerText = "" + index + " : " + element.card + " " + element.value;
        btn.onclick = function() {sendCardInput(index);};
        HCardWrapper.appendChild(btn);
        //<button onclick="sendCardInput()" >send handcardnumber</button><br>

    }
}

function createSession(){
const url = base_api + "/maumau/session/create";
fetch(url, {
  method: "GET",
  headers: {
    "Content-type": "application/json; charset=UTF-8"
  }
})
  .then(response =>
      response.json()
      )
      .then(json => {
          console.log(json);
          localStorage.setItem("session_key", json.session_key);
          localStorage.setItem("player_key", json.player_key);
      });
}

function startSession(){
const url = base_api + "/maumau/session/start"
    + "?sessionkey=" + localStorage.getItem("session_key")
    + "&playerkey="  + localStorage.getItem("player_key");
fetch(url, {
  method: "POST",
  headers: {
    "Content-type": "application/json; charset=UTF-8"
  }
})
  .then(response => {
      console.log(response);
      window.location.href = base_api + "/html/MauMau/MauMau.html";
  }
  );
}

function getSessions(){

const url = base_api + "/maumau/session/all";
fetch(url, {
  method: "GET",
  headers: {
    "Content-type": "application/json; charset=UTF-8"
  }
})
  .then(response =>
      response.json()
      )
      .then(json => {
          console.log(json);
          myForLoop(json.sessions);
      });
}

class Session{
    constructor(session_key, player_count){
        this.session_key = session_key;
        this.player_count = player_count;
    }
}

function myForLoop(items){
    const forWrapper = document.getElementById("myFor");
    forWrapper.innerHTML = "";

    for (let index = 0; index < items.length; index++) {
        const element = items[index];
        let btn = document.createElement("button");
        btn.value = element.session_key;
        btn.innerText = "" + index + " : " + element.session_key;
        btn.onclick = function() {joinSession(this);};
        forWrapper.appendChild(btn);
        //<button onclick="sendCardInput()" >send handcardnumber</button><br>

    }
}

function joinSession(element){
    const session_key = element.value;
    const url = base_api + "/maumau/session/join?sessionkey=" + session_key;
fetch(url, {
  method: "GET",
  headers: {
    "Content-type": "application/json; charset=UTF-8"
  }
})
  .then(response =>
      response.json()
      )
      .then(json => {
          console.log(json);
          localStorage.setItem("session_key", session_key);
          localStorage.setItem("player_key", json.player_key);
      });
}








