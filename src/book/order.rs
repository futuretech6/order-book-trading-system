use std::fmt::Display;

use crate::Price;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    /// Sell
    Ask,
    /// Buy
    Bid,
}

/// Information of an order
///
/// `PartialOrd` and `Ord` is determined by the size of the order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Order {
    pub order_type: OrderType,
    pub owner: u64,
    pub price: Price,
    pub quantity: u64,
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Order {{ order_type: {:?}, owner: {}, price: {}, quantity: {} }}",
            self.order_type, self.owner, self.price, self.quantity
        )
    }
}

impl Order {
    /// Notifies about the execution status of the order.
    ///
    /// If `executed_quantity` is `Some(quantity)`, returns a message indicating
    /// a partial execution with the specified quantity. If
    /// `executed_quantity` is `None`, returns a message indicating that the
    /// order has been fully executed.
    ///
    /// # Arguments
    ///
    /// * `executed_quantity` - An `Option<u64>` representing the quantity that
    ///   was executed. If `None`, the order is considered fully executed.
    ///
    /// # Returns
    ///
    /// A `String` describing the execution status of the order.
    pub fn inform_execution(&self, executed_quantity: Option<u64>) {
        if let Some(quantity) = executed_quantity {
            if quantity > 0 {
                println!("Order partially executed: {} - {} qty", self, quantity)
            } else {
                println!("Order created: {}", self);
            }
        } else {
            println!("Order fully executed: {}", self)
        }
    }
}
