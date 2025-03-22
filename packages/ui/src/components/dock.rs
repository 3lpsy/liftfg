use dioxus::prelude::*;
use fgdb::data::profile::ProfileData;

use crate::{
    icons::{GearIcon, HomeIcon, InboxIcon},
    router,
};
use router::Route;

#[component]
pub fn Dock() -> Element {
    let current_profile_ctx = use_context::<Signal<Option<ProfileData>>>();
    // this should never be none because of the container guard
    let _current_profile_id = use_memo(move || current_profile_ctx().as_ref().map_or(0, |p| p.id));
    let route: Route = use_route();
    let active_tab = match route {
        Route::ProfileIndexView { .. }
        | Route::ProfileShowView { .. }
        | Route::ProfileCreateView { .. } => "settings",
        Route::ProfileWorkoutCreateView { .. }
        | Route::WorkoutCreateView { .. }
        | Route::ProfileWorkoutIndexView { .. } => "workouts",
        _ => "home",
    };
    rsx! {
        div { class: "dock bg-base-300",
            button {
                class: if active_tab == "home" { "dock-active"},
                HomeIcon{}
                span { class: "dock-label", "Session" }
            }
            Link {
                to: Route::ProfileWorkoutIndexView {  } ,
                class: if active_tab == "workouts" { "dock-active"},
                HomeIcon{}
                span { class: "dock-label", "Workouts" }
            }
            button {
                class: if active_tab == "history" { "dock-active"},
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
