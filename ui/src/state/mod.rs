use dioxus::signals::{GlobalSignal, Signal};
use validator::ValidationErrors;

pub static APP_ERRORS: GlobalSignal<Vec<ValidationErrors>> = Signal::global(|| vec![]);

pub struct CurrentProfileId(pub Option<i32>);
