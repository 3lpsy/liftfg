#[cfg(feature = "db")]
use sea_orm::prelude::StringLen;
#[cfg(feature = "db")]
use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
#[cfg(not(feature = "db"))]
use strum::EnumIter as StrumEnumIter;

#[cfg_attr(feature = "db", derive(EnumIter, DeriveActiveEnum))]
#[cfg_attr(not(feature = "db"), derive(StrumEnumIter))]
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")
)]
pub enum MuscleOrderStrategy {
    #[serde(rename = "DETERMINISTIC")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "DETERMINISTIC"))]
    #[default]
    Deterministic,

    #[serde(rename = "LEAST_WORKED")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "LEAST_WORKED"))]
    LeastWorked,

    #[serde(rename = "ROTATING")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "ROTATING"))]
    Rotating,
}

impl MuscleOrderStrategy {
    pub fn from_string(s: &str) -> Self {
        match s {
            "DETERMINISTIC" => Self::Deterministic,
            "LEAST_WORKED" => Self::LeastWorked,
            "ROTATING" => Self::Rotating,
            _ => Self::default(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Deterministic => "DETERMINISTIC".to_string(),
            Self::LeastWorked => "LEAST_WORKED".to_string(),
            Self::Rotating => "ROTATING".to_string(),
        }
    }
}

#[cfg_attr(feature = "db", derive(EnumIter, DeriveActiveEnum))]
#[cfg_attr(not(feature = "db"), derive(StrumEnumIter))]
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")
)]
pub enum ExerciseSplitStrategy {
    #[serde(rename = "SIMPLE")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "SIMPLE"))]
    #[default]
    Simple,

    #[serde(rename = "NO_ADJUST")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "NO_ADJUST"))]
    NoAdjust,
}

impl ExerciseSplitStrategy {
    pub fn from_string(s: &str) -> Self {
        match s {
            "SIMPLE" => Self::Simple,
            "NO_ADJUST" => Self::NoAdjust,
            _ => Self::default(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Simple => "SIMPLE".to_string(),
            Self::NoAdjust => "NO_ADJUST".to_string(),
        }
    }
}

#[cfg_attr(feature = "db", derive(EnumIter, DeriveActiveEnum))]
#[cfg_attr(not(feature = "db"), derive(StrumEnumIter))]
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "db",
    sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")
)]
pub enum ExercisePromptStrategy {
    #[serde(rename = "COMMON_COMPOUND")]
    #[cfg_attr(feature = "db", sea_orm(string_value = "COMMON_COMPOUND"))]
    #[default]
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

impl ExercisePromptStrategy {
    pub fn from_string(s: &str) -> Self {
        match s {
            "COMMON_COMPOUND" => Self::CommonCompound,
            "COMMON_ISOLATION" => Self::CommonIsolation,
            "COMMON_COMPOUND_ROTATE" => Self::CommonCompoundRotate,
            "COMMON_PREVIOUS" => Self::CommonPrevious,
            "CHAOTIC" => Self::Chaotic,
            _ => Self::default(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::CommonCompound => "SIMPLE".to_string(),
            Self::CommonIsolation => "COMMON_ISOLATION".to_string(),
            Self::CommonCompoundRotate => "COMMON_COMPOUND_ROTATE".to_string(),
            Self::CommonPrevious => "COMMON_PREVIOUS".to_string(),
            Self::Chaotic => "CHAOTIC".to_string(),
        }
    }
}
