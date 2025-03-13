use serde::{Deserialize, Serialize};

#[cfg(feature = "db")]
use sea_orm::prelude::StringLen;
#[cfg(feature = "db")]
use sea_orm::{DeriveActiveEnum, EnumIter};

#[cfg_attr(feature = "db", derive(EnumIter, DeriveActiveEnum))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")
)]
pub enum MuscleOrderStrategy {
    #[serde(rename = "DETERMINISTIC")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "DETERMINISTIC"))]
    Deterministic,

    #[serde(rename = "LEAST_WORKED")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "LEAST_WORKED"))]
    LeastWorked,

    #[serde(rename = "ROTATING")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "ROTATING"))]
    Rotating,
}

#[cfg_attr(feature = "db", derive(EnumIter, DeriveActiveEnum))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")
)]
pub enum ExcerciseSplitStrategy {
    #[serde(rename = "SIMPLE")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "SIMPLE"))]
    Simple,

    #[serde(rename = "NO_ADJUST")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "NO_ADJUST"))]
    NoAdjust,
}

#[cfg_attr(feature = "db", derive(EnumIter, DeriveActiveEnum))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")
)]
pub enum ExercisePromptStrategy {
    #[serde(rename = "COMMON_COMPOUND")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "COMMON_COMPOUND"))]
    CommonCompound, // default, prompt for most common first, prefer compounds

    #[serde(rename = "COMMON_ISOLATION")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "COMMON_ISOLATION"))]
    CommonIsolation, // default, prompt for most common first, prefer isolation for warmup

    #[serde(rename = "COMMON_COMPOUND_ROTATE")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "COMMON_COMPOUND_ROTATE"))]
    CommonCompoundRotate, // prompt for the most common but rotate

    #[serde(rename = "COMMON_PREVIOUS")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "COMMON_PREVIOUS"))]
    CommonPrevious, // prefer previously done, prompt for common

    #[serde(rename = "CHAOTIC")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "CHAOTIC"))]
    Chaotic, // truly random
}
