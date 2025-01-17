use dioxus::prelude::*;

mod state;

use state::AppState;

fn main() {
    dioxus::launch(App);
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    // let app_state = use_context_provider(ipc::fetch_user);

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
            button { id: "skip", "skip" }
            button { id: "save", "save!" }
        }
    }
}

#[component]
fn DogApp(breed: String) -> Element {
    rsx! {
        "Breed: {breed}"
    }
}
