#![allow(non_snake_case)]
use crate::components::profile::ProfileCreateForm;
use dioxus::prelude::*;

#[component]
pub fn ProfileCreateOnboardView() -> Element {
    rsx! {
        div { class: "hero min-h-screen bg-base-200",
            div { class: "hero-content flex-col",
                div { class: "text-center",
                    h1 { class: "text-5xl font-bold", "Create Profile" }
                }
                ProfileCreateForm{}
            }
        }
    }
}

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
