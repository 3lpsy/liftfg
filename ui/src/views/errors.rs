#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::state::APP_ERRORS;
#[component]
pub fn Errors() -> Element {
    let app_errors = APP_ERRORS.read();
    rsx! {
        code { "{app_errors:?}"}
    }
}
