
window.addEventListener("load", (event) => {
    getGameState2();
    //loop();
  //log.textContent += "load\n";
});



function getGameState2(){
    setInterval(async function(){
const session_key = localStorage.getItem("session_key");
const player_key = localStorage.getItem("player_key");
const url = base_api + "/maumau/player/state?sessionkey=" + session_key
        + "&playerkey=" + player_key;


const response = await fetch(url, {
  method: "GET",
  headers: {
    "Content-type": "application/json; charset=UTF-8"
  }
});

await response.json().then(json => {
    console.log(json);
    updatePlayerState(json);
}
);



}, 2000);

 }
