use dioxus_logger::tracing::info as console_info;

pub fn info(msg: &str) {
    console_info!(msg); // logs to console via dioxus
    #[cfg(target_arch = "wasm32")]
    {
        use crate::bindings::info as tauri_info;
        use wasm_bindgen::prelude::JsValue;
        tauri_info(JsValue::from_str(msg)); // logs to stdout via IPC
    }
}
