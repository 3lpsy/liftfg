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
use fgdb::data::profile::ProfileData;
use state::CurrentProfileId;
use views::Loading;
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

    let current_profile_id: Signal<CurrentProfileId> = use_signal(|| CurrentProfileId(None));
    use_context_provider(|| current_profile_id.clone());

    let profile: Signal<Option<ProfileData>> = use_signal(|| None);
    use_context_provider(|| profile.clone());

    // All Routes under Container
    // Container will query profile w/ profile id
    // Container uses General state to determine whether to render dock/navbar

    rsx! {
        document::Stylesheet { href: MAIN_CSS },
        Meta {
            name: "viewport",
            content: "viewport-fit=cover"
        },
        SuspenseBoundary {
            fallback: |_| rsx!{
                div {
                    class: "page container mx-auto px-4 sm:px-6 lg:px-8 py-8 sm:py-12 flex flex-col",
                    Loading {}
                }
            },
            Router::<router::Route> {}
        }
    }
}
