use crate::{
    components::{
        modal::Modal,
        modal_select::{ModalSelect, SelectOption},
    },
    hooks::muscle::use_muscles_resource,
};

use dioxus::prelude::*;
use fgdb::data::{
    muscle::{MuscleData, MuscleIndexParams},
    workout::WorkoutData,
};
use fgutils::environment::Platform;
use web_sys::HtmlDialogElement;

#[component]
pub fn WorkoutMuclesBatchEditForms(workout: WorkoutData) -> Element {
    let params_sig = use_signal(MuscleIndexParams::default);
    let (muscles_sig, muscles_res) = use_muscles_resource(params_sig);
    muscles_res.suspend()?;
    let mut selected_muscle_ids: Signal<Vec<String>> = use_signal(|| vec![]);
    let platform = use_context::<Platform>();

    // TODO clone
    let selected_muscles_memo = use_memo(move || {
        muscles_sig()
            .clone()
            .into_iter()
            .filter(|m| selected_muscle_ids().contains(&m.id.to_string()))
            .collect::<Vec<MuscleData>>()
    });

    rsx! {
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
                            selected_muscle_ids.set(ids);
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
                        on_change: move |values: Vec<String>| {
                            selected_muscle_ids.set(values);
                        }
                    }
                }
            }
            ul { class: "list bg-base-100 rounded-box shadow-md",
                for muscle in selected_muscles_memo() {
                    li {
                        class: "list-row",
                        div {
                            class: "text-4xl tabular-nums",
                            "{muscle.id}"
                        }
                        div {
                            class: "flex-1 text-lg sm:text-xl font-semibold opacity-80 text-center uppercase self-center",
                            "{muscle.name}"
                        }

                    }

                }
            }
        }
    }
}
