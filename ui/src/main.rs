// mod state;
mod bindings;
// mod invoke;
// mod jslog;
mod components;
mod icons;
mod logging;
mod router;
mod services;
mod state;
mod views;

use dioxus::prelude::*;
use document::Meta;
use fgdb::data::profile::ProfileResponseData;
use state::CurrentProfileId;
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

    let current_profile_id: Signal<CurrentProfileId> = Signal::new(CurrentProfileId(None));
    use_context_provider(|| current_profile_id.clone());

    let profile: Signal<Option<ProfileResponseData>> = Signal::new(None);
    use_context_provider(|| profile.clone());

    // starts Initializing state on Init Route
    // Init shows loading
    // Init handles callback and redirects to /home, /profile/create, or /errors

    rsx! {
        document::Stylesheet { href: MAIN_CSS },
        Meta {
            name: "viewport",
            content: "viewport-fit=cover"
        },
        Router::<router::Route> {}
    }
}
