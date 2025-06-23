use order_book_trading_system::{Order, OrderType, TradingSystem};

#[test]
fn test_bid_higher_than_ask() {
    let mut trading_system = TradingSystem::new();

    // Add a bid order at price 110
    let bid_order = Order {
        order_type: OrderType::Bid,
        price: 110,
        quantity: 10,
        owner: 1,
    };

    // Add an ask order at price 100 (lower than bid, should execute at ask price)
    let ask_order = Order {
        order_type: OrderType::Ask,
        price: 100,
        quantity: 5,
        owner: 2,
    };

    println!("[*] Adding bid order: {}", bid_order);
    trading_system.handle_order(&bid_order);
    println!("[*] Adding ask order: {}", ask_order);
    trading_system.handle_order(&ask_order);

    assert!(trading_system.bid_orders.contains_key(&110));
    assert_eq!(trading_system.bid_orders.get(&110).unwrap().len(), 1);
    assert_eq!(trading_system.bid_orders.get(&110).unwrap()[0].quantity, 5);

    assert!(trading_system.ask_orders.is_empty());
}

#[test]
fn test_multiple_bids_higher_than_ask() {
    let mut trading_system = TradingSystem::new();

    // Add multiple bid orders at different prices
    let bid_order1 = Order {
        order_type: OrderType::Bid,
        price: 110,
        quantity: 5,
        owner: 1,
    };

    let bid_order2 = Order {
        order_type: OrderType::Bid,
        price: 105,
        quantity: 3,
        owner: 2,
    };

    // Add an ask order that will match both bids
    let ask_order = Order {
        order_type: OrderType::Ask,
        price: 100,
        quantity: 7,
        owner: 3,
    };

    println!("[*] Adding bid order 1: {}", bid_order1);
    trading_system.handle_order(&bid_order1);
    println!("[*] Adding bid order 2: {}", bid_order2);
    trading_system.handle_order(&bid_order2);
    println!("[*] Adding ask order: {}", ask_order);
    trading_system.handle_order(&ask_order);

    assert!(!trading_system.bid_orders.contains_key(&110));
    assert!(trading_system.bid_orders.contains_key(&105));
    assert!(trading_system.bid_orders.get(&105).unwrap()[0].quantity == 1);

    assert!(trading_system.ask_orders.is_empty());
}

#[test]
fn test_multiple_asks_lower_than_bid() {
    let mut trading_system = TradingSystem::new();
    let ask_order1 = Order {
        order_type: OrderType::Ask,
        price: 90,
        quantity: 5,
        owner: 2,
    };

    let ask_order2 = Order {
        order_type: OrderType::Ask,
        price: 95,
        quantity: 5,
        owner: 3,
    };

    let bid_order = Order {
        order_type: OrderType::Bid,
        price: 100,
        quantity: 7,
        owner: 1,
    };

    println!("[*] Adding ask order 1: {}", ask_order1);
    trading_system.handle_order(&ask_order1);
    println!("[*] Adding ask order 2: {}", ask_order2);
    trading_system.handle_order(&ask_order2);
    println!("[*] Adding bid order: {}", bid_order);
    trading_system.handle_order(&bid_order);

    assert!(trading_system.ask_orders.contains_key(&95));
    assert!(!trading_system.ask_orders.contains_key(&90));
    assert_eq!(trading_system.ask_orders.get(&95).unwrap().len(), 1);
    assert_eq!(trading_system.ask_orders.get(&95).unwrap()[0].quantity, 3);

    assert!(trading_system.bid_orders.is_empty());
}
