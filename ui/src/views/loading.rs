#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Loading() -> Element {
    rsx! { h1 { "Loading..." } }
}
