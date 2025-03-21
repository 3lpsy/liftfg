use crate::{router, services::post};
use dioxus::prelude::*;
use fgdb::data::profile::{ProfileData, ProfileStoreData};
use validator::{Validate, ValidationErrors};

#[component]
pub fn ProfileCreateForm() -> Element {
    let mut current_profile_ctx = use_context::<Signal<Option<ProfileData>>>();
    let mut form_data = use_signal(|| ProfileStoreData::default());
    let mut form_errors = use_signal(|| ValidationErrors::new());
    let nav = navigator();
    let error_messages = use_memo(move || {
        form_errors()
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
        div {
            class: "card w-full bg-base-100",
            form {
                class: "card-body",
                onsubmit: move |e| async move {
                    e.prevent_default();
                    let mut form_data = form_data();
                    if let Err(validation_errors) = form_data.validate() {
                        form_errors.set(validation_errors);
                    } else {
                        let current_profile = current_profile_ctx();
                        if current_profile.is_none() {
                            form_data.is_default = Some(true);
                        }
                        match post::<ProfileStoreData, ProfileData>("profile_store", form_data).await {
                            Ok(profile) => {
                                let new_profile_id = profile.id;
                                // if there was no previous current profile, set it
                                match current_profile {
                                    Some(_) => {},
                                    None => {
                                        current_profile_ctx.set(Some(profile));
                                    }
                                }
                                nav.replace(router::Route::WorkoutCreateView { profile_id: new_profile_id as usize });
                            },
                            Err(e) => form_errors.set(e)
                        }
                    }
                    Ok(())
                },
                fieldset {
                    class: "fieldset",
                    // legend { class: "fieldset-legend", "Details"}
                    label {
                        class: "fieldset-label",
                        "Name"
                    }
                    input {
                        class: "input w-full",
                        r#type: "text",
                        placeholder: "Enter your name",
                        value: "{form_data().name}",
                        name: "name",
                        oninput: move |evt| {
                            form_data.with_mut(|data| {
                                data.name = evt.value().clone();
                            });
                        }
                    }
                    label {
                        class: "fieldset-label flex justify-end items-center",
                        "Activate"
                        input {
                            class: "toggle",
                            r#type: "checkbox",
                            // if no current profile exists, no profile has ever been created
                            // so keep checked by default in that case
                            disabled: current_profile_ctx().is_none(),
                            checked: {
                                current_profile_ctx().is_none() || form_data().is_default.unwrap_or(false)
                            },
                            oninput: move |evt| {
                                let v = evt.value().parse::<bool>()?;
                                form_data.write().is_default = Some(v);
                                Ok(())
                            }
                        },
                    }

                    for (field, messages) in error_messages().iter() {
                        li {
                            span { class: "font-semibold", "{field}: " }
                            span { "{messages}" }
                        }
                    }
                    button { class: "mt-2 btn w-full", "Create Profile" }

                }
            }
        }
    }
}
