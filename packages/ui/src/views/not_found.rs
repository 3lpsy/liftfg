#![allow(non_snake_case)]
use dioxus::prelude::*;
use validator::ValidationErrors;

#[component]
pub fn Empty(errors: Option<ValidationErrors>) -> Element {
    rsx! {}
}

#[component]
pub fn NotFoundFallback(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
