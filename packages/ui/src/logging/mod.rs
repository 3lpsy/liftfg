#![allow(dead_code)]

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        dioxus_logger::tracing::info!("{}", msg); // logs to console via dioxus
        crate::bindings::info(wasm_bindgen::prelude::JsValue::from_str(&msg)); // logs to stdout via IPC
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        dioxus_logger::tracing::warn!("{}", msg); // logs to console via dioxus
        crate::bindings::warn(wasm_bindgen::prelude::JsValue::from_str(&msg)); // logs to stdout via IPC
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        dioxus_logger::tracing::error!("{}", msg); // logs to console via dioxus
        crate::bindings::error(wasm_bindgen::prelude::JsValue::from_str(&msg)); // logs to stdout via IPC
    }};
}

// Re-export the macros for easier access
pub use crate::{error, info, warn};
