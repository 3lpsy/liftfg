#![allow(dead_code)]

// #[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "log"])]
    pub fn info(args: JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "log"])]
    pub fn warn(args: JsValue) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "log"])]
    pub fn error(args: JsValue) -> JsValue;
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

// Function signatures for LSP
#[cfg(not(target_arch = "wasm32"))]
pub async fn invoke(_cmd: &str, _args: JsValue) -> Result<JsValue, JsValue> {
    unimplemented!()
}
#[cfg(not(target_arch = "wasm32"))]
pub fn info(_args: JsValue) -> JsValue {
    unimplemented!()
}
#[cfg(not(target_arch = "wasm32"))]
pub fn warn(_args: JsValue) -> JsValue {
    unimplemented!()
}
#[cfg(not(target_arch = "wasm32"))]
pub fn error(_args: JsValue) -> JsValue {
    unimplemented!()
}
