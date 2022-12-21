mod utils;

use wasm_bindgen::prelude::*;
mod gg20_signing;
extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, mpc-wasm!");
}

#[wasm_bindgen]
pub fn sign(message: JsValue, local_share: JsValue, sm_manager_url: JsValue, room_id: JsValue) {
    log!("sign called!");

    let data_to_sign = serde_wasm_bindgen::from_value::<String>(message).unwrap();
    let local_share = serde_wasm_bindgen::from_value::<String>(local_share).unwrap().as_bytes();
    let parties = vec![1, 2];
    let sm_manager_url = serde_wasm_bindgen::from_value::<String>(sm_manager_url).unwrap();
    let room_id = serde_wasm_bindgen::from_value::<String>(room_id).unwrap();

    log!("data_to_sign: {:?}", data_to_sign);
    log!("parties: {:?}", parties);
    log!("sm_manager_url: {:?}", sm_manager_url);
    log!("room_id: {:?}", room_id);
}
