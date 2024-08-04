use std::collections::{BTreeSet, VecDeque};

use crate::book::Order;

pub type Price = u64;

/// Newest order will be pushed to the head of the list, and the oldest order
/// will be popped from the tail
pub type TimePriorityList = VecDeque<Order>;

pub type SizePriorityList = BTreeSet<Order>;
