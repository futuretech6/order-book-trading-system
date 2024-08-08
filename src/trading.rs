use crate::book::{Order, OrderBook, OrderList, OrderType};
use crate::{FifoList, QuantityPriorityList};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum TradingPriorityType {
    // Earliest first
    #[default]
    Time,
    // Largest first
    Quantity,
}

#[derive(Default, Debug)]
pub struct TradingSystem {
    priority_type: TradingPriorityType,
    pub ask_orders: OrderBook,
    pub bid_orders: OrderBook,
}

impl TradingSystem {
    pub fn new(priority_type: TradingPriorityType) -> Self {
        Self {
            priority_type,
            ..Default::default()
        }
    }

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
            match bid_orders {
                OrderList::Time(ref mut orders) => {
                    Self::handle_time_priority_list(orders, &mut remaining_quantity);
                }
                OrderList::Quantity(ref mut orders) => {
                    Self::handle_quantity_priority_list(orders, &mut remaining_quantity);
                }
            }
            if remaining_quantity == 0 {
                break;
            }
        }

        // clear empty order_lists
        self.bid_orders
            .retain(|_price, order_list| !order_list.is_empty());

        // add the remaining orders to the ask_order_book
        if remaining_quantity > 0 {
            let order_list_at_price =
                self.ask_orders
                    .entry(ask_order.price)
                    .or_insert_with(match self.priority_type {
                        TradingPriorityType::Time => OrderList::new_time_priority,
                        TradingPriorityType::Quantity => OrderList::new_quantity_priority,
                    });
            match order_list_at_price {
                OrderList::Time(ref mut orders) => {
                    orders.push_new(*ask_order);
                }
                OrderList::Quantity(ref mut orders) => {
                    orders
                        .entry(ask_order.price)
                        .or_insert_with(FifoList::new)
                        .push_new(*ask_order);
                }
            }
        }
    }

    fn handle_bid(&mut self, bid_order: &Order) {
        let mut remaining_quantity = bid_order.quantity;

        // iterate through the ask orders from the lowest price
        for (_price, ask_orders) in self.ask_orders.iter_mut() {
            match ask_orders {
                OrderList::Time(ref mut orders) => {
                    Self::handle_time_priority_list(orders, &mut remaining_quantity);
                }
                OrderList::Quantity(ref mut orders) => {
                    Self::handle_quantity_priority_list(orders, &mut remaining_quantity);
                }
            }
            if remaining_quantity == 0 {
                break;
            }
        }

        // clear empty order_lists
        self.ask_orders
            .retain(|_price, order_list| !order_list.is_empty());

        // add the remaining orders to the bid_order_book
        if remaining_quantity > 0 {
            let order_list_at_price =
                self.bid_orders
                    .entry(bid_order.price)
                    .or_insert_with(match self.priority_type {
                        TradingPriorityType::Time => OrderList::new_time_priority,
                        TradingPriorityType::Quantity => OrderList::new_quantity_priority,
                    });
            match order_list_at_price {
                OrderList::Time(ref mut orders) => {
                    orders.push_new(*bid_order);
                }
                OrderList::Quantity(ref mut orders) => {
                    orders
                        .entry(bid_order.price)
                        .or_insert_with(FifoList::new)
                        .push_new(*bid_order);
                }
            }
        }
    }

    fn handle_time_priority_list(order_list: &mut FifoList, quantity: &mut u64) {
        while let Some(oldest_order) = order_list.oldest_mut() {
            if *quantity >= oldest_order.quantity {
                *quantity -= oldest_order.quantity;
                order_list.pop_oldest();
            } else {
                // no quantity remaining
                oldest_order.quantity -= *quantity;
                *quantity = 0;
                break;
            }
        }
    }

    fn handle_quantity_priority_list(order_list: &mut QuantityPriorityList, quantity: &mut u64) {
        while let Some(mut largest_entry) = order_list.last_entry() {
            let largest_quantity_orders = largest_entry.get_mut();

            Self::handle_time_priority_list(largest_quantity_orders, quantity);

            // use original quantity as the key, so no need to update
            //
            // if let Some(modified_order) = largest_quantity_orders.pop_front() {
            //     if modified_order.quantity != *largest_entry.key() {
            //         // insert the modified order back to the order_list
            //         order_list
            //             .entry(modified_order.quantity)
            //             .or_insert_with(TimePriorityList::new)
            //             .push_back(modified_order);
            //     }
            // }

            if *quantity == 0 {
                // no quantity remaining
                break;
            }
        }
    }
}
