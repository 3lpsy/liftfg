use crate::{
    icons::{ArrowRight, TrashIcon},
    logging::warn,
    router,
    services::profile::delete_profile,
};
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use fgdb::data::profile::{ProfileData, ProfileDeleteParams};
use validator::ValidationErrors;
use wasm_bindgen::JsCast;
use web_sys::HtmlDialogElement;

#[component]
pub fn ProfileListItem(profile: ProfileData) -> Element {
    let mut current_profile_ctx = use_context::<Signal<Option<ProfileData>>>();
    let current_profile_id = use_memo(move || match current_profile_ctx() {
        Some(p) => p.id,
        None => 0,
    });
    let mut modal = use_signal(|| None::<HtmlDialogElement>);
    let nav = use_navigator();
    let profile_id = profile.id;

    rsx! {
        // doesn't close outside but maybe daisyui bug
        dialog {
            onmounted: move |event| {
                let ele = event.as_web_event().dyn_into::<HtmlDialogElement>().unwrap();
                modal.set(Some(ele));
            },
            id: "confirm-profile-delete",
            class: "modal modal-bottom sm:modal-middle",
            div {
                class: "modal-box",
                h3 { class: "text-lg", "Do you really want to delete the profile?" }
                p {
                    "This action will delete the profile and all related data forever."
                }

                div {
                    class:"modal-action flex justify-between w-full",
                    button {
                        class: "btn btn-warning btn-outline",
                        "Delete!"
                    },
                    form {
                        method:"dialog",
                        button {
                            class:"btn btn-info btn-ghost", "Close"
                        }
                    }

                }
            }
            form {
                method:"dialog",
                class: "modal-backdrop",
                onclick: move |_| {
                    modal().expect("modal signal").close();
                }
            }
        }
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
                        modal().expect("modal signal").show_modal().expect("modal unwrap");
                        // let current_profile = current_profile_ctx();
                        // match delete_profile(ProfileDeleteParams {id: profile_id as i32}).await {
                        //     Ok(deleted) => {
                        //         if let Some(p) = current_profile {
                        //             if p.id == deleted.id {
                        //                 // shouldn't happen but just in case
                        //                 current_profile_ctx.set(None);
                        //                 nav.replace(router::Route::ProfileCreateOnboardView {  });
                        //             } else {
                        //                 nav.replace(router::Route::ProfileIndexView {  });
                        //             }

                        //         } else {
                        //             nav.replace(router::Route::ProfileIndexView {  });
                        //         }
                        //     },
                        //     Err(e) => {
                        //         let mut app_errors = use_context::<Signal<ValidationErrors>>();
                        //         app_errors.set(e.clone());
                        //         nav.push(router::Route::Errors { });
                        //     }
                        // }
                        // Ok(())
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
