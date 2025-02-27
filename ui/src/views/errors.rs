#![allow(non_snake_case)]
use dioxus::prelude::*;
use validator::ValidationErrors;

#[component]
pub fn Errors() -> Element {
    let mut app_errors = use_context::<Signal<ValidationErrors>>();
    use_drop(move || app_errors.set(ValidationErrors::new()));
    rsx! {
        code { "{app_errors.read():?}"}
    }
}
