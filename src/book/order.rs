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
    /// a partial execution with the specified quantity. If `executed_quantity`
    /// is `None`, returns a message indicating that the order has been
    /// fully executed. If `execution_price` is `Some(price)`, the message
    /// will include the execution price.
    ///
    /// # Arguments
    ///
    /// * `executed_quantity` - An `Option<u64>` representing the quantity that
    ///   was executed. If `None`, the order is considered fully executed.
    /// * `execution_price` - An `Option<u64>` representing the price at which
    ///   the order was executed. If `None`, the price is not shown in the
    ///   message.
    ///
    /// # Example
    ///
    /// ```ignore
    /// order.inform_execution(Some(5), Some(100)); // Partially executed at price 100
    /// order.inform_execution(None, Some(100));    // Fully executed at price 100
    /// order.inform_execution(Some(5), None);      // Partially executed, price not shown
    /// order.inform_execution(None, None);         // Fully executed, price not shown
    /// ```
    pub fn inform_execution(&self, executed_quantity: Option<u64>, execution_price: Option<f64>) {
        match (executed_quantity, execution_price) {
            (Some(quantity), Some(price)) if quantity > 0 => {
                println!(
                    "Order partially executed: {}, {} qty at price {}",
                    self, quantity, price
                )
            }
            (None, Some(price)) => {
                println!("Order fully executed: {} at price {}", self, price)
            }
            (Some(_), _) => {
                println!("Order created: {}", self);
            }
            (None, _) => {
                println!("Order fully executed: {}", self)
            }
        }
    }
}
