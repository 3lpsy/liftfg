use dioxus::prelude::*;
use fgdb::data::{
    enums::{ExercisePromptStrategy, ExerciseSplitStrategy, MuscleOrderStrategy},
    muscle::MuscleIndexParams,
    workout::WorkoutStoreData,
};
use strum::IntoEnumIterator;
use validator::{Validate, ValidationErrors};

// TODO
// - Let's just create the workout row first and navigate to edit
// - Then make form reusable with edit and move to component
// - In it's own component will probably be the add WorkoutMuscle rows which are complex
// - Will maybe need to handle defaults better in that component as None will more likely be valid (in UI say it's a override of workout)
//
// pub struct WorkoutMuscleData {
// pub id: i32,
// pub workout_id: i32,
// pub muscle_id: i32,
// pub priority: i32,
// pub volume: i32,
// pub exercise_set_split: Option<i32>,
// pub exercise_prompt_strategy: Option<ExercisePromptStrategy>,

#[component]
pub fn WorkoutCreateView() -> Element {
    let mut form_data = use_signal(|| WorkoutStoreData::default());
    let local_form_errors = use_memo(move || {
        if let Err(errs) = form_data().validate() {
            errs
        } else {
            ValidationErrors::default()
        }
    });
    let server_form_errors = use_signal(|| ValidationErrors::new());
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

    let params_sig = use_signal(|| MuscleIndexParams::default());
    // let nav = navigator();
    rsx! {
        h1 { class: "text-2xl sm:text-3xl font-bold text-base-content", "Create Workout" },
        div {
            class: "divider"
        },
        form {
            onsubmit: move |e| async move {
                e.prevent_default();
                // // need to implement command
                // let mut form_data = form_data();
                // form_data.code = codify(&form_data.name);
                // match post::<WorkoutStoreData, WorkoutData>("workout_store", form_data).await {
                //     Ok(_workout) => {
                //         nav.go_back();
                //     },
                //     Err(e) => server_form_errors.set(e)
                // }

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
                            class: if form_data().name.len() > 0 && has_error("name") {
                                "input w-full input-error"
                            } else {
                                "input w-full"
                            },
                            r#type: "text",
                            placeholder: "Enter Workout Name",
                            value: "{form_data().name}",
                            name: "name",
                            oninput: move |evt| {
                                form_data.with_mut(|data| {
                                    data.name = evt.value().clone();
                                });
                            }
                        }
                        if form_data().name.len() > 0 && has_error("name")  {
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
                            class: if form_data().name.len() > 0 && has_error("name") {
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
                    for (field, messages) in server_error_messages().iter() {
                        li {
                            span { class: "font-semibold", "{field}: " }
                            span { "{messages}" }
                        }
                    }
                }
                button {
                    class: "mt-2 btn w-full",
                    disabled: !local_form_errors().is_empty(),
                    "Create Workout and add Muscle Targets"
                }
                "{form_data:?}"
            }

        }
    }
}
