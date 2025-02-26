#![allow(non_snake_case)]
use crate::components::profile::ProfileList;
use crate::components::profile::{ProfileCreateForm, ProfileEditForm};
use crate::logging::warn;
use crate::router;
use crate::services::profile::{delete_profile, get_profile};
use crate::state::CurrentProfileId;
use crate::views::Loading;
use chrono_tz::Tz;
use dioxus::prelude::*;
use fgdb::data::profile::ProfileShowParams;
use fgdb::data::profile::{ProfileData, ProfileDeleteParams};
use fgutils::dt_human;

#[component]
pub fn ProfileShow(profile_id: usize) -> Element {
    let mut profile_sig: Signal<Option<ProfileData>> = use_signal(|| None);
    let mut current_profile_ctx = use_context::<Signal<Option<ProfileData>>>();
    let mut current_profile_id_ctx = use_context::<Signal<CurrentProfileId>>();

    let profile = use_resource(move || async move {
        get_profile(Some(ProfileShowParams {
            id: Some(profile_id as i32),
            name: None,
        }))
        .await
    })
    .suspend()?;
    let nav = use_navigator();
    use_effect(move || match &*profile.read() {
        Ok(profile) => {
            profile_sig.set(Some(profile.clone()));
        }
        Err(e) => {
            nav.replace(router::Route::ResourceNotFound {
                errors: Some(e.clone()),
            });
        }
    });
    let timezone = use_context::<Signal<Tz>>();
    rsx! {
        match &*profile_sig.read() {
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
                            "Created at: {dt_human(profile.created_at, &(*timezone.read()))}"
                        }
                        p {
                            class: "text-sm text-base-content mb-2",
                            {}
                            "Updated at: {dt_human(profile.updated_at, &(*timezone.read()))}"
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
                                to: router::Route::ProfileEdit { profile_id: profile.id as usize },
                                class: "btn btn-primary w-full",
                                "Edit"
                            }
                            button {
                                class: "btn btn-warning w-full",
                                onclick: move |e| async move {
                                    // TODO cast
                                    match delete_profile(ProfileDeleteParams {id: profile_id as i32}).await {
                                        Ok(deleted) => {
                                            let current_profile = current_profile_ctx.read().clone();
                                            match current_profile {
                                                Some(p) => {
                                                    if p.id == deleted.id {
                                                        current_profile_ctx.set(None);
                                                    }
                                                },
                                                None => {}
                                            }
                                            let current_profile_id = current_profile_id_ctx.read().clone();
                                            match current_profile_id.0 {
                                                Some(pid) => {
                                                    if pid == deleted.id {
                                                        current_profile_id_ctx.set(CurrentProfileId(None));
                                                    }
                                                }
                                                None => {}
                                            }
                                            nav.replace(router::Route::ProfileIndex {  });
                                        },
                                        Err(e) => {
                                            // TODO error handling
                                            warn(&format!("{e:?}"));
                                        }
                                    }
                                },
                                "Delete"
                            }
                            {if !profile.is_default {
                                rsx!(
                                    button {
                                        class: "btn btn-secondary w-full",
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

#[component]
pub fn ProfileIndex() -> Element {
    let profiles_ctx: Signal<Vec<ProfileData>> = use_signal(|| vec![]);
    use_context_provider(|| profiles_ctx.clone());
    rsx! {
        h1 { class: "text-2xl sm:text-3xl font-bold text-base-content", "Profiles" },
        div {
            class: "divider"
        }
        ProfileList{}
    }
}

#[component]
pub fn ProfileCreate() -> Element {
    rsx! {
        div { class: "hero min-h-screen bg-base-200",
            div { class: "hero-content flex-col",
                div { class: "text-center",
                    h1 { class: "text-5xl font-bold", "Create Profile" }
                }
                ProfileCreateForm{}
            }
        }
    }
}

#[component]
pub fn ProfileEdit(profile_id: usize) -> Element {
    let mut profile_sig: Signal<Option<ProfileData>> = use_signal(|| None);
    let profile = use_resource(move || async move {
        get_profile(Some(ProfileShowParams {
            id: Some(profile_id as i32),
            name: None,
        }))
        .await
    })
    .suspend()?;
    let nav = use_navigator();
    use_effect(move || match &*profile.read() {
        Ok(profile) => {
            profile_sig.set(Some(profile.clone()));
        }
        Err(e) => {
            nav.replace(router::Route::ResourceNotFound {
                errors: Some(e.clone()),
            });
        }
    });
    rsx! {
        SuspenseBoundary {
            fallback: |_| rsx! { Loading {  }},
            match &*profile_sig.read() {
                None => rsx! { Loading {}},
                Some(profile) => {
                    rsx! {
                        div { class: "hero bg-base-200",
                            div { class: "hero-content flex-col",
                                div { class: "text-center",
                                    h1 { class: "text-5xl font-bold", "Edit Profile" }
                                }
                                ProfileEditForm { profile: profile.clone()}
                            }
                        }
                    }
                }
            }
        }
    }
}
