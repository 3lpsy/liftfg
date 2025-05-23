use dioxus::prelude::*;
use fgdb::data::{profile::ProfileData, DefaultParams, HasPagination};
use validator::ValidationErrors;

use super::profile_list_item::ProfileListItem;
use crate::services::get;

#[component]
pub fn ProfileList() -> Element {
    let mut profiles_ctx = use_context::<Signal<Vec<ProfileData>>>();
    let pagination = use_signal(|| DefaultParams::default().with_size(20));
    let profiles_reload_trigger = use_signal(|| 0);
    let profiles_res = use_resource(move || async move {
        let _ = profiles_reload_trigger.read();
        get::<DefaultParams, Vec<ProfileData>>("profile_index", Some(pagination())).await
    })
    .suspend()?;

    use_effect(move || match profiles_res() {
        Ok(profiles) => {
            *profiles_ctx.write() = profiles.clone();
        }
        Err(e) => {
            let mut app_errors = use_context::<Signal<ValidationErrors>>();
            app_errors.set(e.clone());
        }
    });

    rsx! {
        ul { class: "list bg-base-100 rounded-box shadow-md",
            for profile in profiles_ctx() {
                ProfileListItem {
                    profile: profile.clone(),
                    profiles_reload_trigger: profiles_reload_trigger
                }
            }
        }
    }
}
