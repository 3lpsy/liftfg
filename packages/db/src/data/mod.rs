pub mod enums;
pub mod profile;
pub mod workout;

#[cfg(feature = "db")]
use fgutils::constants::VALIDATION_DATABASE_FIELD;
use fgutils::constants::VALIDATION_REQUEST_FIELD;
use fgutils::patterns::ALPHA_DASH;
#[cfg(feature = "db")]
use sea_orm::ConnectionTrait;
#[cfg(feature = "db")]
use sea_orm::DbErr;
#[cfg(feature = "db")]
use sea_orm::SelectorTrait;
#[cfg(feature = "db")]
use sea_orm::TransactionError;
use serde::{Deserialize, Serialize};
#[cfg(feature = "db")]
use std::borrow::Cow;
use std::collections::HashMap;
use validator::Validate;
#[cfg(feature = "db")]
use validator::ValidationError;
use validator::ValidationErrors;

// validators requires &'static str
// TODO figure this out
static FIELD_NAMES: &[&str] = &["name", "is_default"];
pub fn field_ref(name: &str) -> &'static str {
    FIELD_NAMES
        .iter()
        .find(|&&x| x == name)
        .unwrap_or(&VALIDATION_REQUEST_FIELD)
}

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

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderDirection {
    #[default]
    Asc,
    Desc,
}

#[cfg(feature = "db")]
impl From<OrderDirection> for sea_orm::Order {
    fn from(direction: OrderDirection) -> Self {
        match direction {
            OrderDirection::Asc => sea_orm::Order::Asc,
            OrderDirection::Desc => sea_orm::Order::Desc,
        }
    }
}
#[derive(Debug, Validate, Serialize, Deserialize, Clone)]
pub struct Order {
    pub direction: OrderDirection,
    #[validate(length(
        min = 1,
        max = 256,
        message = "OrderBy must be between 1 and 256 characters long"
    ))]
    #[validate(regex(path = *ALPHA_DASH, message="Field must only contain alphanumeric or -, ., _ characters"))]
    pub order_by: String,
}
impl Default for Order {
    fn default() -> Self {
        Self {
            direction: OrderDirection::Asc,
            order_by: String::from("id"),
        }
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct Pagination {
    #[validate(range(
        min = 0,
        max = 65536,
        message = "Page must be between 1 and 65536 characters long"
    ))]
    pub page: i32,
    #[validate(range(
        min = 1,
        max = 65536,
        message = "Size must be between 1 and 65536 characters long"
    ))]
    pub size: i32,
}

// fetch_page and cur_page are 0 based
impl Default for Pagination {
    fn default() -> Self {
        Self { page: 0, size: 10 }
    }
}

pub trait HasPagination {
    fn pagination(&mut self) -> &mut Option<Pagination>;
    fn with_page(mut self, page: i32) -> Self
    where
        Self: Sized,
    {
        let pagination = self.pagination();
        *pagination = Some(pagination.take().unwrap_or_default());
        pagination.as_mut().unwrap().page = page;
        self
    }
}

pub trait Includable {}

pub trait HasIncludes<T>
where
    T: Includable,
{
    fn includes(&mut self) -> &mut Option<Vec<T>>;
    fn with_include(mut self, include: T) -> Self
    where
        Self: Sized,
    {
        let includes = self.includes();
        if includes.is_none() {
            *includes = Some(Vec::new());
        }
        includes.as_mut().unwrap().push(include);
        self
    }
}

pub trait HasOrder {
    fn order(&mut self) -> &mut Option<Order>;

    fn with_order(mut self, order: Order) -> Self
    where
        Self: Sized,
    {
        *self.order() = Some(order);
        self
    }

    fn with_order_direction(mut self, direction: OrderDirection) -> Self
    where
        Self: Sized,
    {
        let order = self.order();
        if order.is_none() {
            *order = Some(Order::default());
        }
        order.as_mut().unwrap().direction = direction;
        self
    }

    fn with_order_by(mut self, order_by: String) -> Self
    where
        Self: Sized,
    {
        let order = self.order();
        if order.is_none() {
            *order = Some(Order::default());
        }
        order.as_mut().unwrap().order_by = order_by;
        self
    }
}

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
// fetch_page and cur_page are 0 based
// paginatior is what's returned to user
#[derive(Debug, Serialize, Deserialize)]
pub struct Paginator {
    pub page: i32,
    pub size: i32,
    pub pages: i32,
    pub total: i32,
    pub order: OrderDirection,
}

#[cfg(feature = "db")]
impl Paginator {
    pub async fn from_db_paginator<C, S>(
        paginator: &sea_orm::Paginator<'_, C, S>,
        page: i32,
        size: i32,
        order: OrderDirection,
    ) -> Result<Self, DbValidationErrors>
    where
        C: ConnectionTrait,
        S: SelectorTrait,
    {
        // can't cur_page here
        // TODO casting
        let total_items_and_pages = paginator
            .num_items_and_pages()
            .await
            .map_err(DbValidationErrors::from)?;
        let pages = total_items_and_pages.number_of_pages as i32;
        let total = total_items_and_pages.number_of_items as i32;
        Ok(Paginator {
            page,
            size,
            pages,
            total,
            order,
        })
    }
}

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

#[cfg(feature = "db")]
pub struct DbValidationErrors(DbErr);
#[cfg(feature = "db")]
impl From<DbErr> for DbValidationErrors {
    fn from(err: DbErr) -> Self {
        DbValidationErrors(err)
    }
}
#[cfg(feature = "db")]
impl From<TransactionError<DbErr>> for DbValidationErrors {
    fn from(err: TransactionError<DbErr>) -> Self {
        match err {
            TransactionError::Connection(e) => DbValidationErrors(e),
            TransactionError::Transaction(e) => DbValidationErrors(e),
        }
    }
}
#[cfg(feature = "db")]
impl From<DbValidationErrors> for ValidationErrors {
    // field request (generic where it happened)
    // code database (specific failure about what rule was broken, in this case a generic database rule)
    fn from(wrapper: DbValidationErrors) -> Self {
        // TODO messages in realase
        let db_msg = wrapper.0.to_string();
        let default_msg = db_msg.clone();

        match db_msg {
            msg if msg.contains("UNIQUE constraint failed:") => {
                let field = msg
                    .split("UNIQUE constraint failed:")
                    .nth(1)
                    .and_then(|s| s.trim().split('.').nth(1))
                    .map(|s| s.to_string()) // Convert to owned String
                    .unwrap_or_else(|| VALIDATION_REQUEST_FIELD.to_string());

                ValidationErrors::new().with_error(
                    field_ref(&field),
                    ValidationError::new("unique")
                        .with_message(Cow::from(format!("Field {field} must be unique"))),
                )
            }
            _ => ValidationErrors::new().with_error(
                VALIDATION_REQUEST_FIELD,
                ValidationError::new(VALIDATION_DATABASE_FIELD)
                    .with_message(Cow::from(default_msg)),
            ),
        }
    }
}
