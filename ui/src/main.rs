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

use std::str::FromStr;

use chrono_tz::Tz;
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

    let current_profile: Signal<Option<ProfileData>> = use_signal(|| None);
    use_context_provider(|| current_profile.clone());

    // set theme, wasm only?
    use_hook(move || {
        // Access the document and set attributes on the HTML element
        let html = web_sys::window()
            .expect("no global window exists")
            .document()
            .expect("no document on window")
            .document_element()
            .expect("no document element");
        let _ = html.set_attribute("data-theme", "light");
    });
    // does this need to be signal?
    // set timezone
    let mut timezone: Signal<Tz> = use_signal(|| Tz::America__Chicago);
    use_context_provider(|| timezone.clone());

    use_effect(move || {
        spawn(async move {
            let result =
                document::eval("return Intl.DateTimeFormat().resolvedOptions().timeZone").await;

            match result {
                Ok(value) => {
                    if let Some(tz_str) = value.as_str() {
                        // Extract &str from serde_json::Value
                        match Tz::from_str(tz_str) {
                            Ok(valid_tz) => {
                                logging::info(&format!("{valid_tz:?}"));
                                timezone.set(valid_tz)
                            }
                            Err(_) => logging::info(&format!("Invalid timezone: {}", tz_str)),
                        }
                    } else {
                        logging::info("Failed to extract timezone string");
                    }
                }
                Err(e) => {
                    logging::info(&format!("Error fetching timezone: {:?}", e));
                }
            }
        });
    });

    rsx! {
        document::Stylesheet { href: MAIN_CSS },
        document::Meta {
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
