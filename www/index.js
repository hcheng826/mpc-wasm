import * as wasm from "mpc-wasm";
import { v4 as uuidv4 } from 'uuid';
import axios from "axios";

const signFrom = document.getElementById("sign-button");

const SERVICE_URL = "http://localhost:8002";
const SM_MANAGER_URL = "http://localhost:8000";

signFrom.addEventListener("click", (event) => {
    const localShare = document.getElementById("local-share").value;
    const message = document.getElementById("message").value;
    const room_id = uuidv4();

    // axios.post(`${SERVICE_URL}/sign`, {
    //     msg: message,
    //     room_id: room_id
    // });
    axios.get(SERVICE_URL).then(res => console.log(res));

    wasm.sign(message, localShare, SM_MANAGER_URL, room_id);
})


