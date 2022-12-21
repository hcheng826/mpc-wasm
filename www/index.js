import * as wasm from "mpc-wasm";
import { v4 as uuidv4 } from 'uuid';
import axios from "axios";

const theOtherShareButton = document.getElementById("the-other-share-button");
const smButton = document.getElementById("sm-button");

const SERVICE_URL = "http://localhost:8002";
const SM_MANAGER_URL = "http://localhost:8000";

smButton.addEventListener("click", async (event) => {
    const localShare = document.getElementById("local-share").value;
    const message = document.getElementById("message").value;
    const room_id = uuidv4();

    console.log(await wasm.sm_sign(message, localShare, SM_MANAGER_URL, room_id));
})

theOtherShareButton.addEventListener("click", (event) => {
    axios.get(SERVICE_URL).then(res => console.log(res));
    // axios.post(`${SERVICE_URL}/sign`, {
    //     msg: message,
    //     room_id: room_id
    // });
})
