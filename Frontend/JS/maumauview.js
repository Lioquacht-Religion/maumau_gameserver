
var mau_counter = 0;

function sayMauMauState(MauMauWrapper){
    mau_counter++;
    if (mau_counter > 2){
        mau_counter = 0;
    }
    MauMauWrapper.value = mau_counter;
    if (mau_counter == 0) {
        MauMauWrapper.innerText = "No Mau";
    }
    else if (mau_counter == 1) {
        MauMauWrapper.innerText = "Mau";
    }
    else if (mau_counter == 2) {
        MauMauWrapper.innerText = "MauMau";
    }
}


