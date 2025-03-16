use fgutils::constants::VALIDATION_REQUEST_FIELD;
use serde::{Deserialize, Serialize};
use validator::Validate;

// validators requires &'static str
// TODO figure this out
static FIELD_NAMES: &[&str] = &["name", "is_default"];
pub fn field_ref(name: &str) -> &'static str {
    FIELD_NAMES
        .iter()
        .find(|&&x| x == name)
        .unwrap_or(&VALIDATION_REQUEST_FIELD)
}

pub mod meta;
pub use meta::includes::{HasIncludes, Includable};
pub use meta::order::{HasOrder, Order, OrderDirection};
pub use meta::pagination::{HasPagination, Pagination, Paginator};
pub use meta::request::{
    DefaultDataType, DefaultParamsType, RequestData, RequestableData, RequestableParams,
};
pub use meta::response::{ResponsableData, ResponseData};
pub mod enums;
pub mod profile;
pub mod workout;

#[cfg(feature = "db")]
pub use meta::db::DbValidationErrors;

#[derive(Default, Clone, Debug, Validate, Serialize, Deserialize)]
pub struct DefaultParams {
    pub pagination: Option<Pagination>,
    pub order: Option<Order>,
}

impl HasPagination for DefaultParams {
    fn pagination(&mut self) -> &mut Option<Pagination> {
        &mut self.pagination
    }
}

impl HasOrder for DefaultParams {
    fn order(&mut self) -> &mut Option<Order> {
        &mut self.order
    }
}
