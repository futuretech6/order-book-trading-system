mod order;
mod order_list;

use std::collections::BTreeMap;

pub use self::order::*;
pub use self::order_list::*;
use crate::Price;

/// `OrderBook` contains a mapping of prices to lists of orders at that price.
pub type OrderBook = BTreeMap<Price, OrderList>;
