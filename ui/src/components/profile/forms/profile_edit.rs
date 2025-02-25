use crate::{router, services::profile::update_profile};
use dioxus::prelude::*;
use fgdb::data::profile::{ProfileData, ProfileUpdateData};
use validator::{Validate, ValidationErrors};

#[component]
pub fn ProfileEditForm(profile: ProfileData) -> Element {
    let mut form_data: Signal<ProfileUpdateData> = use_signal(|| profile.clone().into());
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
        form {
            class: "card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100",
            onsubmit: move |e| async move {
                e.prevent_default();
                let data = form_data.read().clone();
                if let Err(validation_errors) = data.validate() {
                    form_errors.set(validation_errors);
                } else {
                    match update_profile((data).clone()).await {
                        Ok(profile) => {
                            form_data.set(profile.into()); // probably unnecssary
                            nav.replace(router::Route::ProfileIndex {  });
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
                        value: "{form_data.read().name.as_ref().map_or_else(String::new, |s| s.clone())}",
                        name: "name",
                        oninput: move |evt| {
                            form_data.with_mut(|data| {
                                data.name = Some(evt.value().clone());
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
                    button { class: "btn btn-primary", "Update Profile" }
                }
            }
        }
    }
}
