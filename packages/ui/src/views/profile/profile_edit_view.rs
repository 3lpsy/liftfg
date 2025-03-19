#![allow(non_snake_case)]
use crate::components::profile::ProfileEditForm;
use crate::router;
use crate::services::profile::{delete_profile, get_profile};
use crate::views::Loading;
use dioxus::prelude::*;
use fgdb::data::profile::ProfileShowParams;
use fgdb::data::profile::{ProfileData, ProfileDeleteParams};
use validator::ValidationErrors;

#[component]
pub fn ProfileEditView(profile_id: usize) -> Element {
    let mut current_profile_ctx = use_context::<Signal<Option<ProfileData>>>();

    let mut profile_sig: Signal<Option<ProfileData>> = use_signal(|| None);
    let profile_res = use_resource(move || async move {
        get_profile(Some(ProfileShowParams {
            id: Some(profile_id as i32),
            name: None,
        }))
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
    rsx! {
        SuspenseBoundary {
            fallback: |_| rsx! { Loading {  }},
            match profile_sig() {
                None => rsx! { Loading {}},
                Some(profile) => {
                    rsx! {
                        h1 { class: "text-2xl sm:text-3xl font-bold text-base-content", "Edit Profile" },
                        div {
                            class: "divider"
                        },
                        ProfileEditForm { profile: profile.clone()},
                        div {
                            class: "divider"
                        },
                        div {
                            class: "flex w-full justify-end md:justify-center px-8",
                            button {
                                class: {
                                    let mut cns = format!("mt-4 btn btn-outline btn-error w-full md:max-w-md");
                                    if let Some(p) = current_profile_ctx() {
                                        if p.id == profile_id as i32 {
                                            cns = format!("{cns} btn-disabled");
                                        }
                                    }
                                    format!("{cns}")
                                },
                                onclick: move |_e| async move {
                                    let current_profile = current_profile_ctx();
                                    match delete_profile(ProfileDeleteParams {id: profile_id as i32}).await {
                                        Ok(deleted) => {
                                            if let Some(p) = current_profile {
                                                if p.id == deleted.id {
                                                    // shouldn't happen but just in case
                                                    current_profile_ctx.set(None);
                                                    nav.replace(router::Route::OnboardProfileCreateView  {  });
                                                } else {
                                                    nav.replace(router::Route::ProfileIndexView {  });
                                                }

                                            } else {
                                                nav.replace(router::Route::ProfileIndexView {  });
                                            }
                                        },
                                        Err(e) => {
                                            let mut app_errors = use_context::<Signal<ValidationErrors>>();
                                            app_errors.set(e.clone());
                                            // nav.push(router::Route::Errors { });
                                        }
                                    }
                                    Ok(())
                                },
                                "Delete"
                            }
                        }

                    }
                }
            }
        }
    }
}
