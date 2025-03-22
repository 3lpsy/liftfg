#![allow(non_snake_case)]

use crate::router::{self};
use crate::services::{get, post};
use crate::views::Loading;
use chrono_tz::Tz;
use dioxus::prelude::*;
use fgdb::data::profile::ProfileShowParams;
use fgdb::data::profile::{ProfileData, ProfileUpdateData};
use fgutils::dt_human;
use validator::ValidationErrors;

#[component]
pub fn ProfileShowView(profile_id: usize) -> Element {
    let mut profile_sig: Signal<Option<ProfileData>> = use_signal(|| None);
    let mut current_profile_ctx = use_context::<Signal<Option<ProfileData>>>();

    let profile_res = use_resource(move || async move {
        get::<ProfileShowParams, ProfileData>(
            "profile_show",
            Some(ProfileShowParams {
                id: Some(profile_id as i32),
                name: None,
            }),
        )
        .await
    })
    .suspend()?;
    let nav = navigator();
    use_effect(move || match profile_res() {
        Ok(profile) => {
            profile_sig.set(Some(profile.clone()));
        }
        Err(e) => {
            let mut app_errors = use_context::<Signal<ValidationErrors>>();
            app_errors.set(e.clone());
            // nav.replace(router::Route::Errors {});
        }
    });
    let timezone = use_context::<Signal<Tz>>();
    rsx! {
        match profile_sig() {
            None => rsx! { Loading {  }},
            Some(profile) => {
                rsx! {
                    div {
                        class: "card bg-base-100 shadow-md rounded-lg p-6",
                        h1 {
                            class: "text-2xl font-bold mb-4 text-base-content",
                            "{profile.name}"
                        }
                        p {
                            class: "text-sm text-base-content mb-2",
                            "Created at: {dt_human(profile.created_at, &timezone())}"
                        }
                        p {
                            class: "text-sm text-base-content mb-2",
                            {}
                            "Updated at: {dt_human(profile.updated_at, &timezone())}"
                        }
                        // Default status information
                        p {
                            class: "text-sm text-base-content mb-4",
                            if profile.is_default {
                                "Default Profile: Yes"
                            } else {
                                "Default Profile: No"
                            }
                        }
                        div {
                            class: "flex flex-col space-y-2",
                            Link {
                                to: router::Route::ProfileEditView { profile_id: profile.id as usize },
                                class: "btn w-full",
                                "Edit Profile"
                            }
                            Link {
                                to: router::Route::ProfileWorkoutCreateView { profile_id: profile.id as usize },
                                class: "btn w-full",
                                "Edit Workouts"
                            }

                            {if !profile.is_default {
                                rsx!(
                                    button {
                                        class: "btn btn-secondary w-full",
                                        onclick: move |_| async move {
                                            match post::<ProfileUpdateData, ProfileData>(
                                                "profile_update", ProfileUpdateData {
                                                id: profile_id as i32,
                                                is_default: Some(true),
                                                name: None
                                            }).await {
                                                Ok(profile) => {
                                                    current_profile_ctx.set(Some(profile));
                                                    nav.replace(router::Route::ProfileIndexView {  });
                                                },
                                                Err(e) => {
                                                    let mut app_errors = use_context::<Signal<ValidationErrors>>();
                                                    app_errors.set(e.clone());
                                                    // nav.replace(router::Route::Errors {  });
                                                }
                                            }
                                        },
                                        "Activate"
                                    }
                                )
                            } else { rsx!() } }
                        }
                    }

                }
            }

        }
    }
}
