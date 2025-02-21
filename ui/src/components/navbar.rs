#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::router::Route;

// use crate::icons::DropDownIcon;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { class: "navbar bg-base-100",
            div { class: "navbar-start",
                // will hold back button
            }
            div { class: "navbar-center",
                Link { class: "btn btn-ghost text-xl", to: Route::Home {}, "LIFTFG" }
            }
            div { class: "navbar-end",
                // profile if exists
                Link { class: "btn", to: Route::Home {}, "Button" }
            }
        }
    }
}
