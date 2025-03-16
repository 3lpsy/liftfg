#[cfg(feature = "db")]
use super::db::DbValidationErrors;
use super::order::OrderDirection;
#[cfg(feature = "db")]
use sea_orm::{ConnectionTrait, SelectorTrait};
use serde::{Deserialize, Serialize};
use validator::Validate;

// pagination is what's provided to api
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
