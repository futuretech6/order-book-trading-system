use std::fmt::Display;

use crate::{SizePriorityList, TimePriorityList};

/// Order list of orders of the same price, either sorted by time or size
#[derive(Debug, Clone, PartialEq)]
pub enum OrderList {
    // Earliest first
    Time(TimePriorityList),
    // Largest first
    Size(SizePriorityList),
}

impl Default for OrderList {
    fn default() -> Self {
        OrderList::Time(TimePriorityList::new())
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
            OrderList::Size(ref orders) => {
                for order in orders {
                    writeln!(f, "Size Priority Order: {:?}", order)?;
                }
            }
        }
        Ok(())
    }
}

impl OrderList {
    pub fn new_time_priority() -> Self {
        Self::Time(TimePriorityList::new())
    }

    pub fn new_size_priority() -> Self {
        Self::Size(SizePriorityList::new())
    }

    pub fn is_empty(&self) -> bool {
        match self {
            OrderList::Time(ref orders) => orders.is_empty(),
            OrderList::Size(ref orders) => orders.is_empty(),
        }
    }
}
