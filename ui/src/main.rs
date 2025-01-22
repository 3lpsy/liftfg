use dioxus::prelude::*;

// mod state;
mod bindings;
mod invoke;
mod log;
use log::info;
use tracing::Level;
use wasm_bindgen::JsValue;

fn main() {
    info("Launching App");
    dioxus::launch(App);
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    // let app_state = use_context_provider(ipc::fetch_user);
    info("Rendering App");
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }

        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
        div { id: "dogview",
            img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
        }
        div { id: "buttons",
            button { id: "skip", "skip2 " }
            button { id: "save", "save4" }
        }
    }
}

#[component]
fn DogApp(breed: String) -> Element {
    rsx! {
        "Breed2: {breed}"
    }
}
