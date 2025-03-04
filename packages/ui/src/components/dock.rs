use dioxus::prelude::*;

use crate::{
    icons::{GearIcon, HomeIcon, InboxIcon},
    router,
};
use router::Route;

#[component]
pub fn Dock() -> Element {
    let route: Route = use_route();
    let active_tab = match route {
        Route::ProfileIndexView { .. }
        | Route::ProfileShowView { .. }
        | Route::ProfileCreateView { .. } => "settings",
        Route::ProgramCreateView { .. } => "programs",
        _ => "home",
    };
    rsx! {
        div { class: "dock bg-base-300",
            button {
                class: if active_tab == "home" { "dock-active"},
                HomeIcon{}
                span { class: "dock-label", "Session" }
            }
            button {
                class: if active_tab == "programs" { "dock-active"},
                HomeIcon{}
                span { class: "dock-label", "Programs" }
            }
            button {
                class: if active_tab == "hsitory" { "dock-active"},
                InboxIcon{},
                span { class: "dock-label", "History" }
            }
            button {
                class: if active_tab == "settings" { "dock-active"},
                GearIcon{},
                span { class: "dock-label", "Settings" }
            }
        }
    }
}
