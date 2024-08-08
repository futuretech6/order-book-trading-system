use order_book_trading_system::{Order, OrderList, OrderType, TradingPriorityType, TradingSystem};

#[test]
fn test_quantity_priority_buy_not_enough() {
    let mut trading_system = TradingSystem::new(TradingPriorityType::Quantity);
    let order1 = Order {
        order_type: OrderType::Ask,
        price: 100,
        quantity: 10,
        owner: 1,
    };
    let order2 = Order {
        order_type: OrderType::Ask,
        price: 100,
        quantity: 5,
        owner: 2,
    };
    let order3 = Order {
        order_type: OrderType::Bid,
        price: 100,
        quantity: 5,
        owner: 3,
    };
    trading_system.handle_order(&order1);
    trading_system.handle_order(&order2);
    trading_system.handle_order(&order3);

    let order_list = trading_system.ask_orders.get(&100).unwrap();
    match order_list {
        OrderList::Time(_) => {
            panic!("OrderPriorityType should be Quantity");
        }
        OrderList::Quantity(ref orders) => {
            let orders: Vec<&Order> = orders.values().flatten().collect();
            assert_eq!(orders.len(), 2);
            assert_eq!(orders[0].quantity, 5);
            assert_eq!(orders[0].owner, 1);
            assert_eq!(orders[1].quantity, 5);
            assert_eq!(orders[1].owner, 2);
        }
    }
}

#[test]
fn test_quantity_priority_sell_not_enough() {
    let mut trading_system = TradingSystem::new(TradingPriorityType::Quantity);
    let order1 = Order {
        order_type: OrderType::Bid,
        price: 100,
        quantity: 10,
        owner: 1,
    };
    let order2 = Order {
        order_type: OrderType::Bid,
        price: 100,
        quantity: 5,
        owner: 2,
    };
    let order3 = Order {
        order_type: OrderType::Ask,
        price: 100,
        quantity: 5,
        owner: 3,
    };
    trading_system.handle_order(&order1);
    trading_system.handle_order(&order2);
    trading_system.handle_order(&order3);

    let order_list = trading_system.bid_orders.get(&100).unwrap();
    match order_list {
        OrderList::Time(_) => {
            panic!("OrderPriorityType should be Quantity");
        }
        OrderList::Quantity(ref orders) => {
            let orders: Vec<&Order> = orders.values().flatten().collect();
            assert_eq!(orders.len(), 2);
            assert_eq!(orders[0].quantity, 5);
            assert_eq!(orders[0].owner, 1);
            assert_eq!(orders[1].quantity, 5);
            assert_eq!(orders[1].owner, 2);
        }
    }
}
