#![allow(non_snake_case)]
use dioxus::prelude::*;
use validator::ValidationErrors;

#[component]
pub fn Errors(errors: ValidationErrors) -> Element {
    rsx! {
        code { "{errors:?}"}
    }
}
