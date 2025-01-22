use crate::bindings::info as bound_info;
use wasm_bindgen::prelude::*;
use dioxus_logger::tracing::info as dioxus_info;

pub fn info(msg: &str) {
    dioxus_info!(msg);
    bound_info(JsValue::from_str(msg));
}
