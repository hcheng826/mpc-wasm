import * as wasm from "mpc-wasm";
import { v4 as uuidv4 } from 'uuid';
import axios from "axios";

const theOtherShareButton = document.getElementById("the-other-share-button");
const smButton = document.getElementById("sm-button");

const SERVICE_URL = "http://localhost:8002";
const SM_MANAGER_URL = "http://localhost:8000";

let room_id = uuidv4();
console.log("room_id: ", room_id);

smButton.addEventListener("click", async (event) => {
    const localShare = document.getElementById("local-share").value;
    const message = document.getElementById("message").value;

    console.log("result in index.js", await wasm.sm_sign(message, localShare, SM_MANAGER_URL, room_id));
})

theOtherShareButton.addEventListener("click", (event) => {
    // axios.get(SERVICE_URL).then(res => console.log(res));
    const message = document.getElementById("message").value;
    axios.post(`${SERVICE_URL}/sign`, {
        msg: message,
        room_id: room_id
    });
})
