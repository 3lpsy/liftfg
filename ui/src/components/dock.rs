use dioxus::prelude::*;

use crate::icons::{GearIcon, HomeIcon, InboxIcon};

#[component]
pub fn Dock() -> Element {
    rsx! {
        div { class: "dock",
            button {
                HomeIcon{}
                span { class: "dock-label", "Session" }
            }
            button {
                HomeIcon{}
                span { class: "dock-label", "Programs" }
            }
            button { class: "dock-active",
                InboxIcon{},
                span { class: "dock-label", "History" }
            }
            button {
                GearIcon{},
                span { class: "dock-label", "Settings" }
            }
        }
    }
}
