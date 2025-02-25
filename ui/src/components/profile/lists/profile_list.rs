use dioxus::prelude::*;
use fgdb::data::{profile::ProfileData, DefaultPaginationParams};

use super::profile_list_item::ProfileListItem;
use crate::{router, services::profile::get_profiles};

#[component]
pub fn ProfileList() -> Element {
    let mut profiles_ctx = use_context::<Signal<Vec<ProfileData>>>();
    let pagination = use_signal(|| DefaultPaginationParams::default());

    let profiles = use_resource(move || async move {
        let p = (*pagination.read()).clone();
        get_profiles(Some(p)).await
    })
    .suspend()?;
    let nav = use_navigator();

    use_effect(move || match &*profiles.read() {
        Ok(profiles) => {
            *profiles_ctx.write() = profiles.clone();
        }
        Err(e) => {
            nav.replace(router::Route::Errors { errors: e.clone() });
        }
    });

    rsx! {
        ul { class: "list bg-base-100 rounded-box shadow-md ",
            for profile in &*profiles_ctx.read() {
                ProfileListItem {profile: profile.clone() }
            }
        }
    }
}
