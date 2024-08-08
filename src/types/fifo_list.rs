use std::collections::VecDeque;
use std::ops::Index;

use crate::book::Order;

/// A FIFO list of orders
///
/// Newest order will be pushed to the head of the list, and the oldest order
/// will be popped from the tail
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FifoList(VecDeque<Order>);

impl FifoList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_new(&mut self, order: Order) {
        self.0.push_back(order);
    }

    pub fn pop_oldest(&mut self) -> Option<Order> {
        self.0.pop_front()
    }

    pub fn oldest_mut(&mut self) -> Option<&mut Order> {
        self.0.front_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for FifoList {
    type Output = Order;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> IntoIterator for &'a FifoList {
    type Item = &'a Order;
    type IntoIter = std::collections::vec_deque::Iter<'a, Order>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
