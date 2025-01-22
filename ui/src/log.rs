use crate::bindings::info as bound_info;
use dioxus_logger::tracing::info as dioxus_info;
use wasm_bindgen::prelude::*;

pub fn info(msg: &str) {
    dioxus_info!(msg);
    bound_info(JsValue::from_str(msg));
}
