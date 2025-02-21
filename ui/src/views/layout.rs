#![allow(non_snake_case)]
use crate::{
    components::{Dock, NavBar},
    logging,
    router::Route,
    services::profile::get_profile,
    state::{CurrentProfileId, APP_ERRORS},
};
use dioxus::prelude::*;
use fgdb::data::profile::ProfileGetParams;

#[component]
pub fn Layout() -> Element {
    logging::info("Rendering Init");

    let current_profile_id = use_context::<Signal<CurrentProfileId>>();

    // depends on current profile id
    let profile = use_resource(move || {
        logging::info("Using profile resource");
        get_profile(Some(ProfileGetParams {
            id: (*current_profile_id.read()).0,
            name: None,
        }))
    });
    let nav = use_navigator();
    // depends on profile context
    use_effect(move || match &*profile.read() {
        Some(Ok(None)) => {
            if let Some(err) = nav.replace("/profile/create") {
                logging::error(&format!("Navigation failed: {:?}", err));
            }
        }
        Some(Err(e)) => {
            APP_ERRORS.write().push(e.clone());
            if let Some(err) = nav.replace("/errors") {
                logging::error(&format!("Navigation failed: {:?}", err));
            }
        }
        Some(Ok(_)) => {
            if let Some(err) = nav.replace("/home") {
                logging::error(&format!("Navigation failed: {:?}", err));
            }
        }
        _ => {}
    });

    rsx! {
        NavBar {},
        Outlet::<Route> {},
        Dock {}
    }
}
