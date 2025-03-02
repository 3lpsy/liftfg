#![allow(non_snake_case)]
use crate::{components::profile::ProfileList, router};
use dioxus::prelude::*;
use fgdb::data::profile::ProfileData;

#[component]
pub fn ProfileIndexView() -> Element {
    let profiles_ctx: Signal<Vec<ProfileData>> = use_signal(|| vec![]);
    use_context_provider(|| profiles_ctx.clone());
    rsx! {
        div {
            class: "flex justify-between items-center",
            h1 { class: "text-2xl sm:text-3xl font-bold text-base-content", "Profiles" },
            Link {
                class: "btn btn-outline",
                to: router::Route::ProfileCreateView  {},
                "New Profile"
            }

        }
        div {
            class: "divider"
        }
        ProfileList{}
    }
}
