mod fifo_list;

use std::collections::BTreeMap;

pub use self::fifo_list::*;

pub type Price = u64;

pub type QuantityPriorityList = BTreeMap<Price, FifoList>;
