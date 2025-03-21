use dioxus::prelude::*;
use fgdb::data::workout::WorkoutData;

#[component]
pub fn WorkoutGridItem(workout: WorkoutData, on_add: EventHandler<WorkoutData>) -> Element {
    let workout_muscles = workout.workout_muscle.clone().unwrap_or_default();
    rsx! {
        div {
            class: "card bg-base-100 card-xs shadow-sm",
            div {
                class: "card-body",
                h2 { class: "card-title",  "{workout.name}" }
                for workout_muscle in workout_muscles.iter() {
                    p {
                        strong {
                        "{workout_muscle.muscle.as_ref().unwrap().name}: "
                        }
                        "{workout_muscle.volume} sets"
                    }
                }
                div {
                    class: "card-actions justify-end",
                    button {
                        class: "btn",
                        onclick: move |_| on_add.call(workout.clone()),
                        "Add"
                    }
                }
            }
        }
    }
}
