use crate::bindings;
use dioxus_logger::tracing;
use wasm_bindgen::prelude::JsValue;
pub fn info(msg: &str) {
    tracing::info!(msg); // logs to console via dioxus
    bindings::info(JsValue::from_str(msg)); // logs to stdout via IPC
}

pub fn warn(msg: &str) {
    tracing::warn!(msg); // logs to console via dioxus
    bindings::warn(JsValue::from_str(msg)); // logs to stdout via IPC
}
