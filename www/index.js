import * as wasm from "mpc-wasm";
import { v4 as uuidv4 } from 'uuid';
import axios from "axios";

const theOtherShareButton = document.getElementById("the-other-share-button");
const smButton = document.getElementById("sm-button");
const httpTestButton = document.getElementById("http-test");
const streamTestButton = document.getElementById("stream-test");

const SERVICE_URL = "http://localhost:8002";
const SM_MANAGER_URL = "http://localhost:8000";

let room_id = uuidv4();
console.log("room_id: ", room_id);

smButton.addEventListener("click", async (event) => {
    const localShare = document.getElementById("local-share").value;
    const message = document.getElementById("message").value;

    console.log("result in index.js", await wasm.sm_sign(message, localShare, SM_MANAGER_URL, room_id));
})

theOtherShareButton.addEventListener("click", async (event) => {
    const message = document.getElementById("message").value;
    await axios.post(`${SERVICE_URL}/sign`, {
        msg: message,
        room_id: room_id
    });
})

httpTestButton.addEventListener("click", async (event) => {
    let res = await wasm.http();
    console.log("res: ", res);
})

streamTestButton.addEventListener("click", async (event) => {
    let res = await wasm.stream();
    console.log("res: ", res);
})
