#![allow(non_snake_case)]
use crate::logging;
use crate::router::Route;
use crate::services::profile::create_profile;
use crate::state::CurrentProfileId;
use dioxus::prelude::*;
use fgdb::data::profile::{ProfileCreateData, ProfileData};
use validator::Validate;
use validator::{ValidationErrors, ValidationErrorsKind};

#[component]
pub fn ProfileIndex() -> Element {
    rsx! { h1 { "Profile Index page" } }
}

#[component]
pub fn ProfileCreate() -> Element {
    let mut current_profile_id_ctx = use_context::<Signal<CurrentProfileId>>();
    let mut profile_ctx = use_context::<Signal<Option<ProfileData>>>();

    let mut form_data = use_signal(|| ProfileCreateData::default());
    let mut form_errors = use_signal(|| ValidationErrors::new());
    let nav = use_navigator();
    let error_messages = use_memo(move || {
        form_errors
            .read()
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                let messages = errors
                    .iter()
                    .map(|e| match &e.message {
                        Some(msg) => msg.to_string(),
                        None => e.to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                (field.clone(), messages)
            })
            .collect::<Vec<_>>()
    });
    rsx! {
        div { class: "hero min-h-screen bg-base-200",
            div { class: "hero-content flex-col",
                div { class: "text-center",
                    h1 { class: "text-5xl font-bold", "Create Profile" }
                }
                form {
                    class: "card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100",
                    onsubmit: move |e| async move {
                        e.prevent_default();
                        let mut form_data = form_data.read().clone();
                        if let Err(validation_errors) = form_data.validate() {
                            form_errors.set(validation_errors);
                        } else {
                            // if none, there is no default
                            let id = (*current_profile_id_ctx.read()).0;
                            if id.is_none() {
                                form_data.is_default = Some(true);
                            }
                            match create_profile((form_data).clone()).await {
                                Ok(profile) => {
                                    let new_profile_id = profile.id;
                                    match id {
                                        Some(_) => {},
                                        None => {
                                            current_profile_id_ctx.set(CurrentProfileId(Some(profile.id)));
                                            profile_ctx.set(Some(profile));
                                        }
                                    }
                                    nav.replace(Route::ProgramCreate { profile_id: new_profile_id as usize });
                                },
                                Err(e) => form_errors.set(e)
                            }
                        }
                    },
                    div { class: "card-body",
                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text", "Name" }
                            }
                            input {
                                class: "input input-bordered",
                                r#type: "text",
                                placeholder: "Enter your name",
                                value: "{form_data.read().name}",
                                name: "name",
                                oninput: move |evt| {
                                    form_data.with_mut(|data| {
                                        data.name = evt.value().clone();
                                    });
                                }
                            }
                        }
                        for (field, messages) in error_messages.read().iter() {
                            li {
                                span { class: "font-semibold", "{field}: " }
                                span { "{messages}" }
                            }
                        }
                        // Submit button
                        div { class: "form-control mt-6",
                            button { class: "btn btn-primary", "Create Profile" }
                        }
                    }
                }
            }
        }
    }
}
