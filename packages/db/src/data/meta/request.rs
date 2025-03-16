use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// request data
// https://doc.rust-lang.org/nomicon/hrtb.html
pub trait RequestableData: for<'de> Deserialize<'de> + Serialize {
    fn as_request(self) -> RequestData<Self, DefaultParamsType> {
        RequestData::new(Some(self), None)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultDataType {}
// seems kind of dangerous....
impl<T> RequestableData for T where T: for<'de> serde::Deserialize<'de> + serde::Serialize {}

// request params
pub type DefaultParamsType = HashMap<String, String>;

pub trait RequestableParams: for<'de> Deserialize<'de> + Serialize {
    fn as_params(self) -> RequestData<DefaultDataType, Self> {
        RequestData {
            data: None,
            params: Some(self),
        }
    }
}
impl<T> RequestableParams for T where T: for<'de> Deserialize<'de> + Serialize {}

// together
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "T: RequestableData, P: RequestableParams")]
pub struct RequestData<T, P> {
    // body data
    pub data: Option<T>,
    // query-ish data
    pub params: Option<P>,
}

impl<T, P> RequestData<T, P>
where
    T: RequestableData,
    P: RequestableParams,
{
    pub fn new(data: Option<T>, params: Option<P>) -> RequestData<T, P> {
        Self { data, params }
    }

    pub fn from_data(data: T) -> RequestData<T, P> {
        RequestData::new(Some(data), None)
    }

    pub fn from_params(params: P) -> RequestData<T, P> {
        RequestData::new(None, Some(params))
    }
}
