#![allow(non_snake_case)]
use crate::logging;
use crate::services::profile::get_profile;
use crate::state::{CurrentProfileId, APP_ERRORS};
use crate::{
    components::{Dock, NavBar},
    router::Route,
};
use dioxus::prelude::*;
use fgdb::data::profile::{ProfileGetParams, ProfileResponseData};

#[component]
pub fn Container() -> Element {
    logging::info("Rendering Container");

    let current_profile_id = use_context::<Signal<CurrentProfileId>>();
    let mut profile_ctx = use_context::<Signal<Option<ProfileResponseData>>>();
    // value only changes when future state changes, not dep?
    let profile = use_resource(move || {
        logging::info("Loading profile resource callback");
        get_profile(Some(ProfileGetParams {
            id: (*current_profile_id.read()).0,
            name: None,
        }))
    })
    .suspend()?;
    let nav = use_navigator();
    // happy path, profile has resolved
    use_hook(move || match &*profile.read() {
        Ok(None) => {
            nav.replace(Route::ProfileCreate {});
        }
        Err(e) => {
            APP_ERRORS.write().push(e.clone());
            nav.replace(Route::Errors {});
        }
        Ok(profile) => {
            *profile_ctx.write() = profile.clone();
        }
    });
    let route: Route = use_route();

    // i could use a protected route strategy with children
    // do i really need and ready state
    // at this point in the render, loading is already done via suspend
    // Borked is handled by redirects to error page
    match route {
        // No navbar/dock
        Route::ProfileCreate {} => rsx! {Outlet::<Route> {}},
        _ => {
            rsx! {
                NavBar {},
                Outlet::<Route> {},
                Dock {}
            }
        }
    }
}
