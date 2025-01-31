use crate::bindings::info as tauri_info;
use dioxus_logger::tracing::info as console_info;
use wasm_bindgen::prelude::JsValue;
pub fn info(msg: &str) {
    console_info!(msg); // logs to console via dioxus
                        // #[cfg(target_arch = "wasm32")]

    tauri_info(JsValue::from_str(msg)); // logs to stdout via IPC
}
