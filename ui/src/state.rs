use dioxus::signals::Signal;

pub struct User {
    name: String,
    email: String,
}

#[derive(Clone, Copy)]
pub struct AppState {
    count: Signal<User>,
}
