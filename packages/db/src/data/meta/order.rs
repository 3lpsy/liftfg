use fgutils::patterns::ALPHA_DASH;
use serde::{Deserialize, Serialize};
use validator::Validate;

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
