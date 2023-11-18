function newSession(){
    let SelectSect =
        document.getElementById("SelectSectWrapper");
    let CreateSect = document.getElementById("CreateSection");
    //CreateSect.classList.replace("NotActive", "Active");
    //SelectSect.classList.replace("Active", "NonActive");
    CreateSect.hidden = false;
    SelectSect.hidden = true;

    console.log("new Session");

}

function createSession(){
const pl_name = document.getElementById("PlayerNameBox");

const max_count = document.getElementById("MaxPlayerCountBox");

const url = base_api + "/maumau/session/create?plname=" + pl_name.value
    + "&maxplayercount=" + max_count.value;
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
          document.getElementById("WaitSection").hidden = false;
          document.getElementById("CreateSection").hidden = true;
      });
}

function startSession(){
const url = base_api + "/maumau/session/start"
    + "?sessionkey=" + localStorage.getItem("session_key")
    + "&playerkey="  + localStorage.getItem("player_key");
fetch(url, {
  method: "POST",
  headers: {
    "Content-type": "application/json; charset=UTF-8",
    "sessionkey": localStorage.getItem("sessionkey")
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
          fillSessionSelection(json.sessions);
      });
}

class Session{
    constructor(session_key, player_count){
        this.session_key = session_key;
        this.player_count = player_count;
    }
}

function fillSessionSelection(items){
    const SessSelectWrapper = document.getElementById("SessionsSelection");
    SessSelectWrapper.innerHTML = "";

    let THeader = document.createElement("tr");
    let Col1 = document.createElement("th");
    Col1.innerText = "Sessionname";
    let Col2 = document.createElement("th");
    Col2.innerText = "current/max Players";
    let Col3 = document.createElement("th");
    Col3.innerText = "State";
    let Col4 = document.createElement("th");

    THeader.appendChild(Col1);
    THeader.appendChild(Col2);
    THeader.appendChild(Col3);
    THeader.appendChild(Col4);
    SessSelectWrapper.appendChild(THeader);

    for (let index = 0; index < items.length; index++) {
        const element = items[index];
        let SessRow = document.createElement("tr");
        let SessionName = document.createElement("td");
        SessionName.innerText = "" + index + " : " + element.session_key;
        let SessionPlCount = document.createElement("td");
        SessionPlCount.innerText = "" + element.player_count;
        let SessionState = document.createElement("td");
        SessionState.innerText = "curState";
        let SessionBtn = document.createElement("td");
        let btn = document.createElement("button");
        btn.value = element.session_key;
        btn.innerText = "Join Session";
        btn.onclick = function() {joinSession(this);};
        SessionBtn.appendChild(btn);
        SessRow.appendChild(SessionName);
        SessRow.appendChild(SessionPlCount);
        SessRow.appendChild(SessionState);
        SessRow.appendChild(SessionBtn);
        SessSelectWrapper.appendChild(SessRow);


    }
}

function joinSession(element){

    const session_key = element.value;
    const url = base_api + "/maumau/session/join?sessionkey=" + session_key;
fetch(url, {
  method: "POST",
  headers: {
    "Content-type": "application/json; charset=UTF-8",
      "sessionkey" : session_key
  }
})
  .then(response =>
      response.json()
      )
      .then(json => {
          console.log(json);
          localStorage.setItem("session_key", session_key);
          localStorage.setItem("player_key", json.player_key);
          document.getElementById("WaitSection").hidden = false;
          document.getElementById("SelectSectWrapper").hidden = true;

      });
}
