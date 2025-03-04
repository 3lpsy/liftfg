#![allow(non_snake_case)]
use crate::components::profile::ProfileCreateForm;
use dioxus::prelude::*;

#[component]
pub fn ProfileCreateView() -> Element {
    rsx! {
        h1 { class: "text-2xl sm:text-3xl font-bold text-base-content", "Create Profile" },
        div {
            class: "divider"
        },
        ProfileCreateForm {},
    }
}
