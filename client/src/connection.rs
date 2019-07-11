use common::protocol::*;
use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::{BinaryType, ErrorEvent, MessageEvent, WebSocket};

use crate::console_log;

pub fn on_data(e: MessageEvent) {
    //let packet = deserialize();
    let packet = e.data();

    let casted: Result<ArrayBuffer, JsValue> = packet.dyn_into();

    match casted {
        Ok(buffer) => {
            let typedbuf: Uint8Array = js_sys::Uint8Array::new(&buffer);
            let mut data: Vec<u8> = vec![0; typedbuf.length() as usize];
            typedbuf.copy_to(data.as_mut_slice());
            //console::log_2(&JsValue::from_str("message event, received buffer: "), &buffer)
            let mex = deserialize(&data).expect("cannot deserialize");
            console_log!("message event, received data: {:?}", mex)
        }
        Err(val) => console_log!("message event, received data: {:?}", val),
    }
}

pub fn start_websocket() -> Result<(), JsValue> {
    // Connect to an echo server
    let ws = WebSocket::new_with_str("ws://localhost:8081", "rust-websocket")?;

    ws.set_binary_type(BinaryType::Arraybuffer);

    // create callback
    let onmessage_callback = Closure::wrap(Box::new(on_data) as Box<dyn FnMut(MessageEvent)>);
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
        console_log!("socket opened");
        match cloned_ws.send_with_u8_array(serialize(Message::Ping).expect("cannot serialize").as_mut_slice()) {
            Ok(_) => console_log!("message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}