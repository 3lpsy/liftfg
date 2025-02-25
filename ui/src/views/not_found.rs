#![allow(non_snake_case)]
use dioxus::prelude::*;
use validator::ValidationErrors;

#[component]
pub fn Empty(errors: Option<ValidationErrors>) -> Element {

    rsx! {

    }
}

#[component]
pub fn ResourceNotFound(errors: Option<ValidationErrors>) -> Element {
    let mut emsg = String::new();
    if let Some(e) = errors {
        emsg = format!("{:?}", e);
    }
    rsx! {
        h1 { "ResourceNotFound not found" }
        code {"{emsg}"}
    }
}

#[component]
pub fn NotFoundFallback(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
