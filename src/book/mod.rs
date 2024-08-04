mod order;
mod order_list;

use std::collections::BTreeMap;

pub use self::order::*;
pub use self::order_list::*;
use crate::Price;

pub type OrderBook = BTreeMap<Price, OrderList>;
