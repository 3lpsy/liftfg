use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

use super::pagination::Paginator;

// https://doc.rust-lang.org/nomicon/hrtb.html
pub trait ResponsableData: Serialize + for<'de> Deserialize<'de> {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "T: ResponsableData")]
pub struct ResponseData<T>
where
    T: ResponsableData,
{
    pub data: Option<T>,
    pub errors: Option<ValidationErrors>,
    pub paginator: Option<Paginator>,
}

impl<T> ResponseData<T>
where
    T: ResponsableData,
{
    pub fn new(
        data: Option<T>,
        errors: Option<ValidationErrors>,
        paginator: Option<Paginator>,
    ) -> ResponseData<T> {
        Self {
            data,
            errors,
            paginator,
        }
    }

    pub fn from_errors(errors: ValidationErrors) -> ResponseData<T> {
        ResponseData::new(None, Some(errors), None)
    }
    pub fn from_data(data: T) -> ResponseData<T> {
        ResponseData::new(Some(data), None, None)
    }
    pub fn from_paginator(data: T, paginator: Paginator) -> ResponseData<T> {
        ResponseData::new(Some(data), None, Some(paginator))
    }
}

impl<T: ResponsableData> From<ValidationErrors> for ResponseData<T> {
    fn from(errors: ValidationErrors) -> Self {
        ResponseData::new(None, Some(errors), None)
    }
}
impl ResponsableData for ValidationErrors {}
