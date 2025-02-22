use dioxus::signals::{GlobalSignal, Signal};
use validator::ValidationErrors;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum State {
    #[default]
    Loading,
    Onboarding,
    Ready,
    Borked,
}

pub static APP_ERRORS: GlobalSignal<Vec<ValidationErrors>> = Signal::global(|| vec![]);
pub static APP_STATE: GlobalSignal<State> = Signal::global(|| State::default());

pub struct CurrentProfileId(pub Option<i32>);
