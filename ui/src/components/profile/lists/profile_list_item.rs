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
            id: "my_modal_1",
            class: "modal modal-bottom sm:modal-middle",
            div {
                class: "modal-box",
                h3 { class: "text-lg", "Hello" }
                div {
                    class:"modal-action",
                    form {
                        method:"dialog",
                        class: "modal-backdrop",
                        button {
                            class:"btn", "Close"
                        }
                    }
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
            Link {
                to: router::Route::ProfileShowView { profile_id: profile.id as usize },
                class: "btn btn-square btn-ghost",
                ArrowRight {}
            }
        }

    }
}
