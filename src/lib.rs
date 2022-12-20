mod utils;

use wasm_bindgen::prelude::*;
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
pub fn sign(local_share: JsValue, message: JsValue){
    log!("sign called!");
    // let example = serde_wasm_bindgen::from_value(val).unwrap();
    log!("local share: {:?}", local_share);
    log!("message: {:?}", message);
}
