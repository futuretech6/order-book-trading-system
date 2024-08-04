use crate::book::{Order, OrderBook, OrderList, OrderType};
use crate::TimePriorityList;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum TradingPriorityType {
    // Earliest first
    #[default]
    Time,
    // Largest first
    Size,
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
        let mut remaining_size = ask_order.size;
        for (_price, bid_orders) in self.bid_orders.iter_mut().rev() {
            match bid_orders {
                OrderList::Time(ref mut orders) => {
                    Self::handle_time_priority_list(orders, &mut remaining_size);
                }
                OrderList::Size(ref mut _orders) => {
                    todo!()
                }
            }
            if remaining_size == 0 {
                break;
            }
        }

        // clear empty order_lists
        self.bid_orders
            .retain(|_price, order_list| !order_list.is_empty());

        // if remaining_size > 0, add the remaining order to the ask_order_book
        if remaining_size > 0 {
            let order_list_at_price =
                self.ask_orders
                    .entry(ask_order.price)
                    .or_insert_with(match self.priority_type {
                        TradingPriorityType::Time => OrderList::new_time_priority,
                        TradingPriorityType::Size => OrderList::new_size_priority,
                    });
            match order_list_at_price {
                OrderList::Time(ref mut orders) => {
                    orders.push_back(*ask_order);
                }
                OrderList::Size(ref mut orders) => {
                    orders.insert(*ask_order);
                }
            }
        }
    }

    fn handle_bid(&mut self, bid_order: &Order) {
        let mut remaining_size = bid_order.size;
        for (_price, ask_orders) in self.ask_orders.iter_mut().rev() {
            match ask_orders {
                OrderList::Time(ref mut orders) => {
                    Self::handle_time_priority_list(orders, &mut remaining_size);
                }
                OrderList::Size(ref mut _orders) => {
                    todo!()
                }
            }
            if remaining_size == 0 {
                break;
            }
        }

        // clear empty order_lists
        self.ask_orders
            .retain(|_price, order_list| !order_list.is_empty());

        // if remaining_size > 0, add the remaining order to the bid_order_book
        if remaining_size > 0 {
            let order_list_at_price =
                self.bid_orders
                    .entry(bid_order.price)
                    .or_insert_with(match self.priority_type {
                        TradingPriorityType::Time => OrderList::new_time_priority,
                        TradingPriorityType::Size => OrderList::new_size_priority,
                    });
            match order_list_at_price {
                OrderList::Time(ref mut orders) => {
                    orders.push_back(*bid_order);
                }
                OrderList::Size(ref mut orders) => {
                    orders.insert(*bid_order);
                }
            }
        }
    }

    fn handle_time_priority_list(order_list: &mut TimePriorityList, remaining_size: &mut u64) {
        if let Some(oldest_order) = order_list.front_mut() {
            if *remaining_size >= oldest_order.size {
                *remaining_size -= oldest_order.size;
                order_list.pop_front();
            } else {
                oldest_order.size -= *remaining_size;
                *remaining_size = 0;
            }
        }
    }
}
