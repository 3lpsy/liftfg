#![allow(non_snake_case)]
use dioxus::prelude::*;
use fgdb::data::profile::ProfileData;

use crate::{icons::ProfileIcon, router::Route};

// use crate::icons::DropDownIcon;

#[component]
pub fn NavBar() -> Element {
    let profile_ctx = use_context::<Signal<Option<ProfileData>>>();

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
                if profile_ctx.read().is_some() {
                    Link {
                        class: "btn btn-square btn-ghost",
                        to: Route::ProfileIndex {},
                        ProfileIcon {}
                    }
                }
            }
        }
    }
}
