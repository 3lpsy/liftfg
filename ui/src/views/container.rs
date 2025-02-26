#![allow(non_snake_case)]
use crate::components::dock::Dock;
use crate::components::navbar::NavBar;
use crate::logging::{self, info};
use crate::services::profile::get_profile;
use crate::state::CurrentProfileId;
use crate::{router::Route, views::Loading};
use dioxus::prelude::*;
use fgdb::data::profile::{ProfileData, ProfileShowParams};

#[component]
pub fn Container() -> Element {
    logging::info("Rendering Container");

    let current_profile_id_ctx = use_context::<Signal<CurrentProfileId>>();
    let mut current_profile_ctx = use_context::<Signal<Option<ProfileData>>>();
    // value only changes when future state changes, not dep?
    let profile = use_resource(move || async move {
        logging::info("Loading profile resource callback");
        // implicitly use is_default if current is None, in that case, check the error
        get_profile(Some(ProfileShowParams {
            id: (*current_profile_id_ctx.read()).0,
            name: None,
        }))
        .await
    })
    .suspend()?;

    let nav = use_navigator();
    // happy path, profile has resolved
    use_effect(move || match &*profile.read() {
        Ok(profile) => {
            info("Updating current profile in container");
            *current_profile_ctx.write() = Some(profile.clone());
            // nav.replace(Route::Home {});
            // we don't really want to nav home do we
        }
        Err(e) => {
            let should_create_profile = e.field_errors().iter().any(|(field, errors)| {
                field == "is_default" && errors.iter().any(|err| err.code == "exists")
            });
            if should_create_profile {
                nav.replace(Route::ProfileCreate {});
            } else {
                nav.replace(Route::Errors { errors: e.clone() });
            }
        }
    });
    // this causes rerender on route change
    let route: Route = use_route();
    rsx! {
        match route {
            Route::ProfileCreate {} => rsx! {
                SuspenseBoundary {
                    fallback: |_| rsx!{ Loading {  }},
                    Outlet::<Route> {}
                    "Route: {route}"
                }
            },
            _ => {
                rsx! {
                    NavBar {},
                    div {
                        class: "page container mx-auto flex flex-col",
                        SuspenseBoundary {
                            fallback: |_| rsx!{
                                div {
                                    class: "flex items-center justify-center flex-1",
                                    Loading {  }
                                }
                            },
                            div {
                                class: "mx-4 my-2",
                                Outlet::<Route> {}
                                "Route: {route}"
                            }

                        }

                    }
                    Dock {}
                }
            }
        }

    }
}
