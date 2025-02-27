use dioxus::prelude::*;
use fgdb::data::profile::ProfileData;

use crate::{icons::ArrowRight, router};

#[component]
pub fn ProfileListItem(profile: ProfileData) -> Element {
    rsx! {
        li {
            class: "list-row",
            div {
                class: "text-4xl tabular-nums",
                "{profile.id}"
            }
            div {
                class: "flex-1 text-lg sm:text-xl font-semibold opacity-80 text-center uppercase self-center",
                if profile.is_default {
                    "{profile.name} (ACTIVE)"
                } else {
                    "{profile.name}"
                }
            }
            Link {
                to: router::Route::ProfileShowView { profile_id: profile.id as usize },
                class: "btn btn-square btn-ghost",
                ArrowRight {}
            }
        }

    }
}
