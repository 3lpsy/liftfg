mod container;
mod home;
mod loading;
mod not_found;
mod onboard;
mod profile;
pub use container::Container;
pub use home::Home;
pub use loading::Loading;
pub use not_found::{Empty, NotFoundFallback};
pub use onboard::{OnboardIndexView, OnboardProfileCreateView, OnboardTermsIndexView};
pub use profile::{ProfileCreateView, ProfileEditView, ProfileIndexView, ProfileShowView};

pub mod profile_workout;
pub use profile_workout::ProfileWorkoutCreateView;
pub mod workout;
pub use workout::{WorkoutCreateView, WorkoutIndexView};
