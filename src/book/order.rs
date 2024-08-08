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
