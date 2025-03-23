use dioxus::prelude::*;
use fgdb::data::{
    enums::{ExercisePromptStrategy, ExerciseSplitStrategy, MuscleOrderStrategy},
    workout::{WorkoutData, WorkoutUpdateData},
};
use fgutils::codify;
use validator::{Validate, ValidationErrors};

use crate::{router, services::post};

#[component]
pub fn WorkoutEditForm(workout: WorkoutData) -> Element {
    let mut form_data = use_signal(|| WorkoutUpdateData::from(workout.clone()));
    let form_data_full = use_memo(move || {
        let mut fd = form_data();
        let fd_name = fd.name.clone().unwrap_or_default();
        let w_name = workout.clone().name;
        if w_name != fd_name {
            // changed
            fd.code = Some(codify(&fd_name));
        }
        return fd;
    });
    let local_form_errors = use_memo(move || {
        if let Err(errs) = form_data_full().validate() {
            errs
        } else {
            ValidationErrors::default()
        }
    });
    let mut server_form_errors = use_signal(|| ValidationErrors::new());
    let server_error_messages = use_memo(move || {
        server_form_errors()
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

    let has_error = move |key: &str| local_form_errors().field_errors().contains_key(key);
    let get_errors = move |key: &str| {
        local_form_errors()
            .field_errors()
            .iter()
            .filter_map(|(field, errors)| {
                if field != key {
                    None
                } else {
                    Some(
                        errors
                            .iter()
                            .map(|e| match &e.message {
                                Some(msg) => (e.code.clone(), msg.to_string()),
                                None => (e.code.clone(), e.to_string()),
                            })
                            .collect::<Vec<_>>(),
                    )
                }
            })
            .flatten()
            .collect::<Vec<_>>()
    };

    rsx! {
        form {
            onsubmit: move |e| async move {
                e.prevent_default();
                tracing::info!("Form submit");
                let form_data = form_data_full();
                match post::<WorkoutUpdateData, WorkoutData>("workout_update", form_data).await {
                    Ok(workout) => {
                        tracing::info!("workout updated {:?}", workout);
                    },
                    Err(e) => {
                        tracing::info!("workout errr {:?}", &e);
                        server_form_errors.set(e)
                    }
                }
                tracing::info!("Form submit comp");

                Ok(())
            },
            div {
                class: "card w-full bg-base-100",
                div {
                    class: "card-body p-1",
                    fieldset {
                        class: "fieldset",
                        label {
                            class: "fieldset-label",
                            "Name"
                        }
                        input {
                            class: if has_error("name") {
                                "input w-full input-error"
                            } else {
                                "input w-full"
                            },
                            r#type: "text",
                            placeholder: "Enter Workout Name",
                            value: "{form_data().name.map_or_else(String::new, |v|v.clone())}",
                            name: "name",
                            oninput: move |evt| {
                                form_data.with_mut(|data| {
                                    data.name = Some(evt.value().clone());
                                });
                            }
                        }
                        if has_error("name")  {
                            for (code, message) in get_errors("name") {
                                li {
                                    span { class: "font-semibold", "{code}: " }
                                    span { "{message}" }
                                }
                            }
                        }
                        label {
                            class: "fieldset-label",
                            "Default Set Split "
                        }
                        input {
                            class: if has_error("exercise_set_split") {
                                "input w-full input-error"
                            } else {
                                "input w-full"
                            },
                            r#type: "number",
                            value: "{form_data().exercise_set_split.clone().unwrap_or_default()}",
                            name: "exercise_set_split",
                            min: Some(1),
                            max: Some(32),
                            inputmode: Some("numeric"),
                            oninput: move |evt| {
                                let num = evt.value().parse::<i32>().unwrap_or(3);
                                form_data.with_mut(|data| {
                                    data.exercise_set_split = Some(num);
                                });
                                Ok(())
                            }
                        }
                        if has_error("exercise_set_split")  {
                            for (code, message) in get_errors("exercise_set_split") {
                                li {
                                    span { class: "font-semibold", "{code}: " }
                                    span { "{message}" }
                                }
                            }
                        }

                        label {
                            class: "fieldset-label",
                            "Muscle Order Strategy"
                        }
                        select {
                            class: "select w-full",
                            value: "{form_data().muscle_order_strategy.clone().to_owned().unwrap_or_default().to_string()}",
                            oninput: move |evt| form_data.with_mut(|data| {
                                data.muscle_order_strategy = Some(MuscleOrderStrategy::from_string(&evt.value()));
                            }),
                            for mos in MuscleOrderStrategy::iter() {
                                option {
                                    value: "{mos.to_string()}",
                                    selected: Some(form_data().muscle_order_strategy.clone().to_owned().unwrap_or_default() == mos),

                                    "{mos:?}"
                                }
                            }
                        }

                        label {
                            class: "fieldset-label",
                            "Exercise Split Strategy"
                        }
                        select {
                            class: "select w-full",
                            value: "{form_data().exercise_split_strategy.clone().to_owned().unwrap_or_default().to_string()}",
                            oninput: move |evt| form_data.with_mut(|data| {
                                data.exercise_split_strategy = Some(ExerciseSplitStrategy::from_string(&evt.value()));
                            }),
                            for mos in ExerciseSplitStrategy::iter() {
                                option {
                                    value: "{mos.to_string()}",
                                    selected: Some(form_data().exercise_split_strategy.clone().to_owned().unwrap_or_default() == mos),
                                    "{mos:?}"
                                }
                            }
                        }
                        label {
                            class: "fieldset-label",
                            "Exercise Prompt Strategy"
                        }
                        select {
                            class: "select w-full",
                            value: "{form_data().exercise_prompt_strategy.clone().to_owned().unwrap_or_default().to_string()}",
                            oninput: move |evt| form_data.with_mut(|data| {
                                data.exercise_prompt_strategy = Some(ExercisePromptStrategy::from_string(&evt.value()));
                            }),
                            for mos in ExercisePromptStrategy::iter() {
                                option {
                                    value: "{mos.to_string()}",
                                    selected: Some(form_data().exercise_prompt_strategy.clone().to_owned().unwrap_or_default() == mos),

                                    "{mos:?}"
                                }
                            }
                        }
                    }
                    // TODO: make pretty / consistent
                    for (field, messages) in server_error_messages().iter() {
                        li {
                            span { class: "font-semibold", "{field}: " }
                            span { "{messages}" }
                        }
                    }
                }
                button {
                    r#type: "submit",
                    class: "mt-2 btn w-full",
                    disabled: !local_form_errors().is_empty(),
                    "Save Changes"
                }
                p{"{form_data_full:?}"}
                p {"{local_form_errors:?}"}
                p {"{server_form_errors:?}"}

            }
        } // form
    }
}
