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
        for (_bid_price, bid_orders) in self
            .bid_orders
            .iter_mut()
            .filter(|(&bid_price, _)| bid_price >= ask_order.price)
            .rev()
        {
            Self::handle_order_list(bid_orders, &mut remaining_quantity, &ask_order.price);
            if remaining_quantity == 0 {
                break;
            }
        }

        // clear empty order_lists
        self.bid_orders
            .retain(|_price, order_list| !order_list.is_empty());

        // add the remaining orders to the ask_order_book
        if remaining_quantity > 0 {
            ask_order.inform_execution(
                Some(ask_order.quantity - remaining_quantity),
                Some(ask_order.price as f64),
            );
            let order_list_at_price = self.ask_orders.entry(ask_order.price).or_default();
            order_list_at_price.push_new(*ask_order);
        } else {
            ask_order.inform_execution(None, Some(ask_order.price as f64));
        }
    }

    fn handle_bid(&mut self, bid_order: &Order) {
        let mut remaining_quantity = bid_order.quantity;

        let mut execution_price_qty: u128 = 0;

        // iterate through the ask orders from the lowest price
        for (ask_price, ask_orders) in self
            .ask_orders
            .iter_mut()
            .filter(|(&ask_price, _)| bid_order.price >= ask_price)
        {
            // For bid orders, execution price is the ask price
            let handled_quantity =
                Self::handle_order_list(ask_orders, &mut remaining_quantity, ask_price);
            execution_price_qty += *ask_price as u128 * handled_quantity as u128;
            if remaining_quantity == 0 {
                break;
            }
        }

        // clear empty order_lists
        self.ask_orders
            .retain(|_price, order_list| !order_list.is_empty());

        let avg_execution_price =
            execution_price_qty as f64 / (bid_order.quantity - remaining_quantity) as f64;

        // add the remaining orders to the bid_order_book
        if remaining_quantity > 0 {
            bid_order.inform_execution(
                Some(bid_order.quantity - remaining_quantity),
                Some(avg_execution_price),
            );
            let order_list_at_price = self.bid_orders.entry(bid_order.price).or_default();
            order_list_at_price.push_new(*bid_order);
        } else {
            bid_order.inform_execution(None, Some(avg_execution_price));
        }
    }

    fn handle_order_list(
        order_list: &mut FifoList,
        quantity: &mut u64,
        execution_price: &u64,
    ) -> u64 {
        let mut handled_quantity = 0;
        while let Some(oldest_order) = order_list.oldest_mut() {
            if *quantity >= oldest_order.quantity {
                let _executed_qty = oldest_order.quantity;
                *quantity -= oldest_order.quantity;
                handled_quantity += oldest_order.quantity;
                oldest_order.inform_execution(None, Some(*execution_price as f64));
                order_list.pop_oldest();
            } else {
                // no quantity remaining
                oldest_order.inform_execution(Some(*quantity), Some(*execution_price as f64));
                oldest_order.quantity -= *quantity;
                handled_quantity += *quantity;
                *quantity = 0;
                break;
            }
        }
        handled_quantity
    }
}
