#![allow(non_snake_case)]
use crate::logging;
use crate::services::profile::get_profile;
use crate::state::{CurrentProfileId, State, APP_ERRORS};
use crate::views::Loading;
use crate::{
    components::{Dock, NavBar},
    router::Route,
    state::APP_STATE,
};
use dioxus::prelude::*;
use fgdb::data::profile::ProfileGetParams;

#[component]
pub fn Container() -> Element {
    logging::info("Rendering Container");

    let current_profile_id = use_context::<Signal<CurrentProfileId>>();

    // depends on current profile id
    let profile = use_resource(move || {
        logging::info("Loading profile resource callback");
        get_profile(Some(ProfileGetParams {
            id: (*current_profile_id.read()).0,
            name: None,
        }))
    });
    let nav = use_navigator();
    use_effect(move || match &*profile.read() {
        Some(Ok(None)) => {
            if let Some(err) = nav.replace(Route::ProfileCreate {}) {
                *APP_STATE.write() = State::Borked;
                logging::error(&format!("Navigation failed: {:?}", err));
            } else {
                *APP_STATE.write() = State::Onboarding;
            }
        }
        Some(Err(e)) => {
            APP_ERRORS.write().push(e.clone());
            if let Some(err) = nav.replace(Route::Errors {}) {
                logging::error(&format!("Navigation failed: {:?}", err));
            }
            *APP_STATE.write() = State::Borked;
        }
        Some(Ok(_)) => {
            if let Some(err) = nav.replace(Route::Home {}) {
                *APP_STATE.write() = State::Borked;
                logging::error(&format!("Navigation failed: {:?}", err));
            } else {
                *APP_STATE.write() = State::Ready;
            }
        }
        _ => {}
    });
    rsx! {
        if *APP_STATE.read() == State::Loading {
            Loading {}
        } else {
            if *APP_STATE.read() == State::Ready {
                NavBar {},
            }
            Outlet::<Route> {},
            if *APP_STATE.read() == State::Ready {
                Dock {}
            }
        }
    }
}
