use crate::logging::info;
use fgdb::data::{
    profile::{
        ProfileData, ProfileDeleteParams, ProfileShowParams, ProfileStoreData, ProfileUpdateData,
    },
    DefaultPaginationParams,
};
use gloo_timers::future::sleep;
use std::time::Duration;
use validator::ValidationErrors;

use super::{get, post};

pub async fn get_profiles(
    args: Option<DefaultPaginationParams>,
) -> Result<Vec<ProfileData>, ValidationErrors> {
    info("Getting Profiles");
    // sleep(Duration::from_secs(10)).await;
    get("profile_index", args).await
}

// converted to params
pub async fn get_profile(args: Option<ProfileShowParams>) -> Result<ProfileData, ValidationErrors> {
    info("Getting Profile");
    // sleep(Duration::from_secs(1)).await;
    get("profile_show", args).await
}

// converted to request w/ data
pub async fn create_profile(args: ProfileStoreData) -> Result<ProfileData, ValidationErrors> {
    info("Creating Profile");
    // sleep(Duration::from_secs(1)).await;
    post("profile_store", args).await
}

pub async fn update_profile(args: ProfileUpdateData) -> Result<ProfileData, ValidationErrors> {
    info("Updating Profile");
    // sleep(Duration::from_secs(1)).await;
    post("profile_update", args).await
}

pub async fn delete_profile(args: ProfileDeleteParams) -> Result<ProfileData, ValidationErrors> {
    info("Deleting Profile");
    sleep(Duration::from_secs(1)).await;
    get("profile_delete", Some(args)).await
}
