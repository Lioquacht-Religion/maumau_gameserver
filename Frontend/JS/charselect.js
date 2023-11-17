function changeCharPic(dir){
    var imgEl = document.getElementById("charportrait");
    var ifrEl = document.getElementById("charDescr");

    var hrefArr = ["F_FHpage.html", "F_HHpage.html", "F_WHpage.html", "F_PHpage.html", "F_OBpage.html"]; 


//DONE :TODO: Add edge control
//TODO: adding changing changing link

    var strSrc = imgEl.src;
    
    var lstr = strSrc.split("CharPortraits/CharPortrait")[1];
    lstr = lstr.split(".p")[0]
    var curimgIndex = parseInt(lstr);

    if(dir < 0 && curimgIndex <= 0){curimgIndex= 5}
    else  
    if(dir > 0 && curimgIndex >= 4){curimgIndex= -1}

        imgEl.src = '../Media/CharPortraits/CharPortrait' +`${curimgIndex+dir}` + '.png';
        ifrEl.src = '../html/'+hrefArr[curimgIndex+dir];


}



