#![allow(non_snake_case)]
use crate::logging;
use crate::services::profile::get_profile;
use crate::state::{CurrentProfileId, APP_ERRORS};
use crate::{
    components::{Dock, NavBar},
    router::Route,
};
use dioxus::prelude::*;
use fgdb::data::profile::{ProfileData, ProfileGetParams};

#[component]
pub fn Container() -> Element {
    logging::info("Rendering Container");

    let current_profile_id_ctx = use_context::<Signal<CurrentProfileId>>();
    let mut profile_ctx = use_context::<Signal<Option<ProfileData>>>();
    // value only changes when future state changes, not dep?
    let profile = use_resource(move || {
        logging::info("Loading profile resource callback");
        // implicitly use is_default if current is None, in that case, check the error
        get_profile(Some(ProfileGetParams {
            id: (*current_profile_id_ctx.read()).0,
            name: None,
        }))
    })
    .suspend()?;

    let nav = use_navigator();
    // happy path, profile has resolved
    use_hook(move || match &*profile.read() {
        Ok(profile) => {
            *profile_ctx.write() = Some(profile.clone());
            return nav.replace(Route::Home {});
        }
        Err(e) => {
            let should_create_profile = e.field_errors().iter().any(|(field, errors)| {
                field == "is_default" && errors.iter().any(|err| err.code == "exists")
            });
            if should_create_profile {
                return nav.replace(Route::ProfileCreate {});
            } else {
                APP_ERRORS.write().push(e.clone());
                return nav.replace(Route::Errors {});
            }
        }
    });
    // this causes rerender on route change
    let route: Route = use_route();
    // i could use a protected route strategy with children
    // do i really need and ready state
    // at this point in the render, loading is already done via suspend
    // Borked is handled by redirects to error page
    rsx! {
        match route {
            Route::ProfileCreate {} => rsx! {Outlet::<Route> {}},
            _ => {
                rsx! {
                    NavBar {},
                    Outlet::<Route> {},
                    Dock {}
                }
            }
        }
        "Route: {route}",

    }
}
