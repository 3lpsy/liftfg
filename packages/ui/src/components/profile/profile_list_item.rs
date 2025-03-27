use crate::{
    components::modal::Modal,
    icons::{ArrowRight, TrashIcon},
    router,
    services::post,
};
use dioxus::prelude::*;
use fgdb::data::profile::{ProfileData, ProfileDeleteParams};
use validator::ValidationErrors;
use web_sys::HtmlDialogElement;

#[component]
pub fn ProfileListItem(profile: ProfileData, profiles_reload_trigger: Signal<i32>) -> Element {
    let mut current_profile_ctx = use_context::<Signal<Option<ProfileData>>>();
    let current_profile_id = use_memo(move || match current_profile_ctx() {
        Some(p) => p.id,
        None => 0,
    });
    let nav = navigator();
    let profile_id = profile.id;
    let modal_ref = use_signal(|| None::<HtmlDialogElement>);
    rsx! {
        // doesn't close outside but maybe daisyui bug
        Modal {
            id: "profile-list-item-modal-id-{profile_id}",
            title: "Do you really want to delete the profile?",
            description: "This action will delete the profile and all related data forever.",
            modal_ref: modal_ref,
            div {
                class: "modal-action flex justify-between w-full",
                button {
                    class: "btn btn-warning btn-outline",
                    onclick: move |_| async move {
                        let current_profile = current_profile_ctx();
                        match post::<ProfileDeleteParams, ProfileData>("profile_delete", ProfileDeleteParams {id: profile_id as i32}).await {
                            Ok(deleted) => {
                                if let Some(p) = current_profile {
                                    if p.id == deleted.id {
                                        // shouldn't happen but just in case
                                        current_profile_ctx.set(None);
                                        if let Some(r) = modal_ref() {
                                            r.close();
                                        }
                                        nav.replace(router::Route::OnboardProfileCreateView  {  });
                                    } else {
                                        if let Some(r) = modal_ref() {
                                            r.close();
                                        }
                                        profiles_reload_trigger.set(profiles_reload_trigger() + 1);
                                        nav.replace(router::Route::ProfileIndexView {  });
                                    }
                                } else {
                                    if let Some(r) = modal_ref() {
                                        r.close();
                                    }
                                    profiles_reload_trigger.set(profiles_reload_trigger() + 1);
                                    nav.replace(router::Route::ProfileIndexView {  });
                                }
                            },
                            Err(e) => {
                                if let Some(r) = modal_ref() {
                                    r.close();
                                }
                                let mut app_errors = use_context::<Signal<ValidationErrors>>();
                                app_errors.set(e.clone());
                                // nav.push(router::Route::Errors { });
                            }
                        }
                    },
                    "Delete!"
                },
            }

        },
        li {
            class: "list-row",
            div {
                class: "text-4xl tabular-nums",
                "{profile_id}"
            }
            div {
                class: "flex-1 text-lg sm:text-xl font-semibold opacity-80 text-center uppercase self-center",
                if profile.is_default {
                    "{profile.name} (ACTIVE)"
                } else {
                    "{profile.name}"
                }
            }
            if current_profile_id != profile_id {
                button {
                    class: "btn btn-square btn-ghost",
                    onclick: move |_e| async move {
                        if let Some(r) = modal_ref() {
                            r.show_modal().expect("Unwrap Modal");
                        }
                    },
                    TrashIcon {}
                }
            }
            Link {
                to: router::Route::ProfileShowView { profile_id: profile.id as usize },
                class: "btn btn-square btn-ghost",
                ArrowRight {}
            }
        }

    }
}
