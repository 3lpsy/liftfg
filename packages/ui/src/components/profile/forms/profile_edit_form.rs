use crate::{router, services::profile::update_profile};
use dioxus::prelude::*;
use fgdb::data::profile::{ProfileData, ProfileUpdateData};
use validator::{Validate, ValidationErrors};

#[component]
pub fn ProfileEditForm(profile: ProfileData) -> Element {
    let mut form_data: Signal<ProfileUpdateData> = use_signal(|| profile.clone().into());
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
                    if let Err(validation_errors) = form_data().validate() {
                        form_errors.set(validation_errors);
                    } else {
                        match update_profile(form_data()).await {
                            Ok(profile) => {
                                form_data.set(profile.into()); // probably unnecssary
                                nav.replace(router::Route::ProfileIndexView {  });
                            },
                            Err(e) => form_errors.set(e)
                        }
                    }
                },
                fieldset {
                    class: "fieldset",
                    // legend { class: "fieldset-legend", "Details"}
                    label { class: "fieldset-label",
                        "Name"
                    }
                    input {
                        class: "input w-full",
                        r#type: "text",
                        placeholder: "Enter your name",
                        value: "{form_data().name.map_or_else(String::new, |s| s.clone())}",
                        name: "name",
                        oninput: move |evt| {
                            form_data.with_mut(|data| {
                                data.name = Some(evt.value().clone());
                            });
                        }
                    }
                }
                for (field, messages) in error_messages().iter() {
                    p {
                        class: "fieldset-label",
                        span { class: "font-semibold", "{field}: " }
                        span { "{messages}" }
                    }
                }
                button { class: "mt-2 btn w-full", "Update Profile" }
            }



        }
    }
}
