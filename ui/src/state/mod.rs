use dioxus::signals::Signal;

pub struct Profile {
    name: String,
}

#[derive(Clone, Copy)]
pub struct AppState {
    count: Signal<Profile>,
}
