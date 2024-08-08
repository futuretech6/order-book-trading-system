use std::fmt::Display;

use crate::{FifoList, QuantityPriorityList};

/// Order list of orders of the same price, either sorted by time or size
#[derive(Debug, Clone, PartialEq)]
pub enum OrderList {
    // Earliest first
    Time(FifoList),
    // Largest first
    Quantity(QuantityPriorityList),
}

impl Default for OrderList {
    fn default() -> Self {
        OrderList::Time(FifoList::new())
    }
}

impl Display for OrderList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderList::Time(ref orders) => {
                for order in orders {
                    writeln!(f, "Time Priority Order: {:?}", order)?;
                }
            }
            OrderList::Quantity(ref orders) => {
                for order in orders {
                    writeln!(f, "Quantity Priority Order: {:?}", order)?;
                }
            }
        }
        Ok(())
    }
}

impl OrderList {
    pub fn new_time_priority() -> Self {
        Self::Time(FifoList::new())
    }

    pub fn new_quantity_priority() -> Self {
        Self::Quantity(QuantityPriorityList::new())
    }

    pub fn is_empty(&self) -> bool {
        match self {
            OrderList::Time(ref orders) => orders.is_empty(),
            OrderList::Quantity(ref orders) => orders.is_empty(),
        }
    }
}
