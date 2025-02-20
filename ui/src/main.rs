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
use state::AppDataState;
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
    // maybe just change to use_context_provider and use_context from children?
    // Maybe not, though there's only one parent component, use_context_provider is more
    // for if you have multiple of the same parent compnoent
    //
    // maybe SuspenseBoundary?
    let _init_state = use_resource(|| async move {
        AppDataState::load(None).await;
    });
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Router::<router::Route> {}
    }
}
