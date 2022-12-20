import * as wasm from "mpc-wasm";

const signFrom = document.getElementById("sign-button");

signFrom.addEventListener("click", (event) => {
    const localShare = document.getElementById("local-share").value;
    const message = document.getElementById("message").value;
    wasm.sign(localShare, message);
})


