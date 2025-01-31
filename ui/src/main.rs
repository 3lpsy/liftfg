use dioxus::prelude::*;

// mod state;
mod bindings;
// mod invoke;
// mod jslog;
mod components;
mod logging;
mod router;
mod views;

// dx serve --platform desktop: Target is not wasm32 and tauri.core does not exist

fn main() {
    logging::info("Launching App");
    dioxus::launch(App);
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
// const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
fn App() -> Element {
    // First we want to query the local db (directly or via Tauri) and see if we have a user
    // let app_state = use_context_provider(ipc::fetch_user);
    logging::info("Rendering App");
    rsx! {
        // automatically set UTF and viewport
        document::Stylesheet { href: MAIN_CSS }
        // document::Meta {
        //     name: "test2",
        //     content: "Test3"
        // }
        Router::<router::Route> {}

    }
}
