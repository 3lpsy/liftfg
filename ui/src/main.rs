// mod state;
mod bindings;
// mod invoke;
// mod jslog;
mod components;
mod logging;
mod router;
mod state;
mod views;

use dioxus::prelude::*;
use state::{AppDataState, RESOURCES_RUNNING};
// use state::AppState;

// dx serve --platform desktop: Target is not wasm32 and tauri.core does not exist

fn main() {
    logging::info("Launching App");
    dioxus::launch(App);
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
fn App() -> Element {
    logging::info("Rendering App");
    let _init_state = use_resource(|| async move {
        RESOURCES_RUNNING.write().insert("load_profile".to_string());
        // Load profile into global state
        AppDataState::load(None).await;
        // Remove from global state once complete
        RESOURCES_RUNNING.write().remove("load_profile");
    });
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Router::<router::Route> {}
    }
}
