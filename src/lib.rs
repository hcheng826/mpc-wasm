mod utils;
mod gg20_signing;
use wasm_bindgen::prelude::*;
mod http;

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
pub async fn sm_sign(
    message: JsValue,
    local_share: JsValue,
    sm_manager_url: JsValue,
    room_id: JsValue,
) -> Result<JsValue, JsValue> {
    log!("sign called!");

    let data_to_sign = serde_wasm_bindgen::from_value::<String>(message).unwrap();
    let local_share = serde_wasm_bindgen::from_value::<String>(local_share).unwrap();
    let parties = vec![1, 2];
    let sm_manager_url = serde_wasm_bindgen::from_value::<String>(sm_manager_url).unwrap();
    let room_id = serde_wasm_bindgen::from_value::<String>(room_id).unwrap();

    // log!("data_to_sign: {:?}", data_to_sign);
    // log!("parties: {:?}", parties);
    // log!("sm_manager_url: {:?}", sm_manager_url);
    // log!("room_id: {:?}", room_id);
    // log!("local_share2: {:?}", local_share2);
    let result = gg20_signing::sign(
        data_to_sign,
        local_share.replace("\\", ""),
        parties,
        surf::Url::parse(&sm_manager_url).unwrap(),
        room_id,
    )
    .await;

    log!("{:?}", result);

    Ok(JsValue::from(result.unwrap()))
}

#[wasm_bindgen]
pub async fn http() -> Result<JsValue, JsValue> {
    let res = http::http_test().await;

    log!("http: {:?}", res);
    Ok(JsValue::from(res.unwrap()))
}
