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
                        "Unlike other trackers, LIftFG focuses on what muscles you want to target for a workout and will prompt you with exercises that you can choose from for that muscle while you workout. "
                    }
                    p {class: "mb-5",
                        "Exercises can be easily skipped during a workout and LiftFG will learn not to prompt you for that exercise again at that Gym."
                    }
                    p { class: "mb-5",
                        "Open source, local only, and 100% private."
                    }
                    Link {
                        to: router::Route::OnboardProfileCreateView {  },
                        class: "btn btn-outline w-full",
                        "Get Started"
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
