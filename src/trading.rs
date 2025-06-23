use crate::book::{Order, OrderBook, OrderType};
use crate::FifoList;

/// Implementation of the `TradingSystem`, which manages the order book and
/// handles order matching.
///
/// # Order Matching Logic
///
/// - Orders are matched based on price-time priority.
/// - Ask orders are matched against the highest available bids.
/// - Bid orders are matched against the lowest available asks.
/// - Remaining unmatched quantities are added to the respective order books.
/// - Empty order lists are removed from the order books after matching.
///
/// **Notes**
///
/// When an ask (sell) order has price lower than the highest bid (buy) order,
/// the execution price is the ask price. For example, the incoming ask price is
/// 100 and the highest bid price is 110, the execution price is 100.
#[derive(Default, Debug)]
pub struct TradingSystem {
    pub ask_orders: OrderBook,
    pub bid_orders: OrderBook,
}

impl TradingSystem {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Handles an incoming order by delegating to either `handle_ask` or
    /// `handle_bid`   depending on the order type.
    pub fn handle_order(&mut self, order: &Order) {
        match order.order_type {
            OrderType::Ask => self.handle_ask(order),
            OrderType::Bid => self.handle_bid(order),
        }
    }

    fn handle_ask(&mut self, ask_order: &Order) {
        let mut remaining_quantity = ask_order.quantity;

        // iterate through the bid orders from the highest price
        for (_price, bid_orders) in self.bid_orders.iter_mut().rev() {
            Self::handle_order_list(bid_orders, &mut remaining_quantity);
            if remaining_quantity == 0 {
                break;
            }
        }

        // clear empty order_lists
        self.bid_orders
            .retain(|_price, order_list| !order_list.is_empty());

        // add the remaining orders to the ask_order_book
        if remaining_quantity > 0 {
            ask_order.inform_execution(Some(ask_order.quantity - remaining_quantity));
            let order_list_at_price = self.ask_orders.entry(ask_order.price).or_default();
            order_list_at_price.push_new(*ask_order);
        } else {
            ask_order.inform_execution(None);
        }
    }

    fn handle_bid(&mut self, bid_order: &Order) {
        let mut remaining_quantity = bid_order.quantity;

        // iterate through the ask orders from the lowest price
        for (_price, ask_orders) in self.ask_orders.iter_mut() {
            Self::handle_order_list(ask_orders, &mut remaining_quantity);
            if remaining_quantity == 0 {
                break;
            }
        }

        // clear empty order_lists
        self.ask_orders
            .retain(|_price, order_list| !order_list.is_empty());

        // add the remaining orders to the bid_order_book
        if remaining_quantity > 0 {
            bid_order.inform_execution(Some(bid_order.quantity - remaining_quantity));
            let order_list_at_price = self.bid_orders.entry(bid_order.price).or_default();
            order_list_at_price.push_new(*bid_order);
        } else {
            bid_order.inform_execution(None);
        }
    }

    fn handle_order_list(order_list: &mut FifoList, quantity: &mut u64) {
        while let Some(oldest_order) = order_list.oldest_mut() {
            if *quantity >= oldest_order.quantity {
                *quantity -= oldest_order.quantity;
                oldest_order.inform_execution(None);
                order_list.pop_oldest();
            } else {
                // no quantity remaining
                oldest_order.inform_execution(Some(*quantity));
                oldest_order.quantity -= *quantity;
                *quantity = 0;
                break;
            }
        }
    }
}
