use crate::{
    components::modal_select::{ModalSelect, SelectOption},
    hooks::muscle::use_muscles_resource,
};

use dioxus::prelude::*;
use fgdb::data::{
    enums::ExercisePromptStrategy,
    muscle::{MuscleData, MuscleIndexParams},
    workout::WorkoutData,
    workout_muscle::WorkoutMuscleStoreData,
};
use fgutils::environment::Platform;

#[component]
pub fn WorkoutMuclesBatchEditForms(workout: WorkoutData) -> Element {
    let params_sig = use_signal(MuscleIndexParams::default);
    let (muscles_sig, muscles_res) = use_muscles_resource(params_sig);
    muscles_res.suspend()?;
    let mut selected_muscle_ids: Signal<Vec<String>> = use_signal(|| vec![]);
    let platform = use_context::<Platform>();

    // How this should work:
    // Pull workout_muscle via resource by workout.id
    // The select statement syncs with DB, creating entries w/ default if they don't exist
    //   Mutli select has an (Add Duplicate) label when already exists
    // and merging current
    // For loops over workout_muscle (from db)
    //   Each row in workout muscle is edit form for workout_muscle
    //

    // TODO clone
    let selected_muscles_memo = use_memo(move || {
        muscles_sig()
            .clone()
            .into_iter()
            .filter(|m| selected_muscle_ids().contains(&m.id.to_string()))
            .collect::<Vec<MuscleData>>()
    });

    let mut batch_form_data: Signal<Vec<WorkoutMuscleStoreData>> = use_signal(Vec::new);
    // TODO: clones
    let batch_form_data_memo = use_memo(move || {
        let muscles = muscles_sig().clone(); // Clone once instead of per iteration
        batch_form_data()
            .iter()
            .filter_map(|f| {
                muscles
                    .iter()
                    .find(|m| m.id == f.muscle_id)
                    .map(|muscle| (f.clone(), muscle.clone()))
            })
            .collect::<Vec<_>>()
    });

    // why mut?
    let mut on_muscles_add = move |ids: Vec<String>| {
        for id in ids.iter() {
            if !batch_form_data()
                .iter()
                .any(|form_data| form_data.muscle_id.to_string() == *id)
            {
                batch_form_data.write().push(WorkoutMuscleStoreData {
                    muscle_id: id.parse::<i32>().unwrap(),
                    workout_id: workout.id,
                    ..Default::default()
                })
            }
        }
        selected_muscle_ids.set(ids);
    };

    rsx! {
        p {
            "{batch_form_data():?}"
        }
        div {
            fieldset {
                class: "fieldset",
                legend { class: "fieldset-legend", "Select Muscles" },
                if platform.is_mobile() {
                    select {
                        class: "select w-full",
                        multiple: Some(true),
                        oninput: move |e| {
                            let ids = e.values().get("options").unwrap().iter().map(|x|x.to_string()).collect::<Vec<String>>();
                            on_muscles_add(ids);
                        },
                        for muscle in muscles_sig() {
                            option {
                                selected: selected_muscle_ids().contains(&muscle.id.to_string()),
                                value: "{muscle.id}",
                                "{muscle.name}"
                            }
                        }
                    }
                } else {
                    ModalSelect {
                        id: "x",
                        name:"test1",
                        options: muscles_sig().clone().iter().map(|m| SelectOption {
                            value: m.id.to_string(),
                            label: m.name.clone()
                        }).collect(),
                        initial_selection: selected_muscle_ids(),
                        synced_selection: Some(selected_muscle_ids),
                        on_change: move |ids: Vec<String>| {
                            on_muscles_add(ids);
                        }
                    }
                }
            }
            // enum WorkoutMuscle {
            //     Table,
            //     Id,
            //     WorkoutId,
            //     MuscleId,
            //     Volume,
            //     Priority,
            //     ExerciseSetSplit,
            //     ExercisePromptStrategy,
            // }

            ul { class: "list bg-base-100 rounded-box shadow-md space-y-4",
                for (form_data, muscle) in batch_form_data_memo() {
                    li {
                        class: "list-row p-4 bg-base-200 rounded-lg shadow w-full",
                        form {
                            class: "flex flex-col gap-2 w-full",
                            onsubmit: move |event| {
                                event.prevent_default();
                            },
                            div {
                                class: "text-lg sm:text-xl font-semibold opacity-80 uppercase",
                                "{muscle.name}"
                            }
                            label {
                                class: "label",
                                span { class: "label-text", "Volume" }
                                input {
                                    class: "input input-bordered",
                                    r#type: "number",
                                    min: "1",
                                    max: "64",
                                    value: "{form_data.volume}",
                                    oninput: move |event| {
                                        let value = event.value().parse::<i32>().unwrap_or(form_data.volume);
                                        batch_form_data.with_mut(|data| {
                                            if let Some(entry) = data.iter_mut().find(|d| d.muscle_id == muscle.id) {
                                                entry.volume = value;
                                            }
                                        });
                                    }
                                }
                            }

                            label {
                                class: "label",
                                span { class: "label-text", "Priority" }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "number",
                                    min: "1",
                                    max: "10",
                                    value: "{form_data.priority}",
                                    oninput: move |event| {
                                        let value = event.value().parse::<i32>().unwrap_or(form_data.priority);
                                        batch_form_data.with_mut(|data| {
                                            if let Some(entry) = data.iter_mut().find(|d| d.muscle_id == muscle.id) {
                                                entry.priority = value;
                                            }
                                        });
                                    }
                                }
                            }

                            label {
                                class: "label",
                                span { class: "label-text", "Exercise Set Split (Optional)" }
                                input {
                                    class: "input input-bordered w-full",
                                    r#type: "number",
                                    min: "0",
                                    max: "64",
                                    value: "{form_data.exercise_set_split.unwrap_or_default()}",
                                    oninput: move |event| {
                                        let value = event.value().parse::<i32>().ok();
                                        batch_form_data.with_mut(|data| {
                                            if let Some(entry) = data.iter_mut().find(|d| d.muscle_id == muscle.id) {
                                                entry.exercise_set_split = value;
                                            }
                                        });
                                    }
                                }
                            }

                            label {
                                class: "label",
                                span { class: "label-text", "Exercise Prompt Strategy" }
                                select {
                                    class: "select select-bordered w-full",
                                    oninput: move |event| {
                                        let value = event.value();
                                        batch_form_data.with_mut(|data| {
                                            if let Some(entry) = data.iter_mut().find(|d| d.muscle_id == muscle.id) {
                                                entry.exercise_prompt_strategy = Some(ExercisePromptStrategy::from_string(&value));
                                            }
                                        });
                                    },
                                    for strategy in ExercisePromptStrategy::iter() {
                                        option {
                                            value: "{strategy.to_string()}",
                                            selected: form_data.exercise_prompt_strategy.as_ref() == Some(&strategy),
                                            "{strategy.to_string()}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

        }
    }
}
