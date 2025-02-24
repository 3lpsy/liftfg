#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Loading() -> Element {
    rsx! {
        // Full-screen container with a subtle background
        div {
            class: "flex flex-col items-center justify-center min-h-screen bg-base-200",
            // Container for the spinner and text
            div {
                class: "flex flex-col items-center",
                // DaisyUI-inspired spinner: a rounded border that spins
                div {
                    class: "w-16 h-16 border-t-4 border-b-4 border-primary rounded-full animate-spin",
                }
                // Loading text below the spinner
                h1 {
                    class: "mt-4 text-xl font-semibold text-primary",
                    "Loading..."
                }
            }
        }
    }
}
