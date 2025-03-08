#![allow(non_snake_case)]
use crate::{components::profile::ProfileCreateForm, router};
use dioxus::prelude::*;

#[component]
pub fn OnboardIndexView() -> Element {
    rsx! {
        div { class: "hero h-full align-center",
            div { class: "hero-content text-center",
                div { class: "max-w-sm",
                    h1 { class: "text-5xl font-bold", "LiftFG" },
                    p {class: "my-5",
                        "Unlike other trackers, LIftFG focuses on what muscles you want to target for a session and will prompt you with exercises that you can choose from for that muscle while you session. "
                    }
                    p {class: "mb-5",
                        "Exercises can be easily skipped during a session and LiftFG will learn not to prompt you for that exercise again at that Gym."
                    }
                    p { class: "mb-5",
                        "Open source, local only, and 100% private."
                    }
                    Link {
                        to: router::Route::OnboardTermsIndexView {  },
                        class: "btn btn-outline w-full",
                        "Get Started"
                    }
                }
            }
        }
    }
}

#[component]
pub fn OnboardTermsIndexView() -> Element {
    rsx! {
        div { class: "hero h-full align-center",
            div { class: "hero-content text-center",
                div { class: "max-w-sm",
                    h1 { class: "text-5xl font-bold", "Terminology" },
                    p {class: "my-5",
                        "Workout: A workout is your template for what you want to work for that day. You can construct an overall program by adding multiple workouts to your profile and choose which workout you want to do when starting a session."
                    }
                    p {class: "mb-5",
                        "Session: A session is you're actual lifting session. You start a session by selecting a workout. Sessions track your progress for given exercises."
                    }
                    p { class: "mb-5",
                        "Gym: LiftFG learns what exercises to show you based off what equipment your gym supports. Simply tell LiftFG that a given exercise is not supported when prompted and you will not be prompted again. If you only ever use a single Gym and rarely go elsewhere, you can set a default."
                    }
                    Link {
                        to: router::Route::OnboardProfileCreateView {  },
                        class: "btn btn-outline w-full",
                        "Create a Profile"
                    }
                }
            }
        }
    }
}

#[component]
pub fn OnboardProfileCreateView() -> Element {
    rsx! {
        div { class: "hero h-full align-center",
            div { class: "hero-content flex-col",
                div { class: "text-center",
                    h1 { class: "text-5xl font-bold", "Profile Setup" }
                }
                ProfileCreateForm{}
            }
        }
    }
}
