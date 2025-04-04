#![allow(non_snake_case)]
use dioxus::prelude::*;
use fgdb::data::profile::ProfileData;

use crate::{
    icons::{ArrowLeft, ProfileIcon},
    router::Route,
};

// use crate::icons::DropDownIcon;

#[component]
pub fn NavBar() -> Element {
    let profile_ctx = use_context::<Signal<Option<ProfileData>>>();
    let nav = navigator();
    let route: Route = use_route();
    let can_go_back = match route {
        Route::OnboardIndexView {} => false,
        _ => nav.can_go_back(),
    };
    rsx! {
        div { class: "navbar sticky top-0 bg-base-300 pb-0 px-0 pt-[env(safe-area-inset-top)]",
            div { class: "navbar-start",
                if can_go_back {
                    a {
                        class: "btn btn-square btn-ghost",
                        onclick: move |_e| nav.go_back(),
                        ArrowLeft{}
                    }
                }
            }
            div { class: "navbar-center",
                if can_go_back {
                    Link { class: "btn btn-ghost text-xl", to: Route::Home {}, "{route}" }
                } else {
                    span { class: "text-xl font-bold", "{route}" }
                }
            }
            div { class: "navbar-end",
                // profile if exists
                if profile_ctx().is_some() {
                    Link {
                        class: "btn btn-square btn-ghost",
                        to: Route::ProfileIndexView {},
                        ProfileIcon {}
                    }
                }
            }
        }
    }
}
