use order_book_trading_system::{Order, OrderType, TradingSystem};

#[test]
fn test_buy_not_enough_to_consume_first() {
    let mut trading_system = TradingSystem::new();
    let order1 = Order {
        order_type: OrderType::Ask,
        price: 100,
        quantity: 5,
        owner: 1,
    };
    let order2 = Order {
        order_type: OrderType::Ask,
        price: 100,
        quantity: 10,
        owner: 2,
    };
    let order3 = Order {
        order_type: OrderType::Bid,
        price: 100,
        quantity: 3,
        owner: 3,
    };
    println!("[*] Adding order: {}", order1);
    trading_system.handle_order(&order1);
    println!("[*] Adding order: {}", order2);
    trading_system.handle_order(&order2);
    println!("[*] Adding order: {}", order3);
    trading_system.handle_order(&order3);

    let orders = trading_system.ask_orders.get(&100).unwrap();

    assert_eq!(orders.len(), 2);
    assert_eq!(orders[0].quantity, 2);
    assert_eq!(orders[0].owner, 1);
    assert_eq!(orders[1].quantity, 10);
    assert_eq!(orders[1].owner, 2);
}

#[test]
fn test_buy_exact_to_consume_first() {
    let mut trading_system = TradingSystem::new();
    let order1 = Order {
        order_type: OrderType::Ask,
        price: 100,
        quantity: 5,
        owner: 1,
    };
    let order2 = Order {
        order_type: OrderType::Ask,
        price: 100,
        quantity: 10,
        owner: 2,
    };
    let order3 = Order {
        order_type: OrderType::Bid,
        price: 100,
        quantity: 5,
        owner: 3,
    };
    println!("[*] Adding order: {}", order1);
    trading_system.handle_order(&order1);
    println!("[*] Adding order: {}", order2);
    trading_system.handle_order(&order2);
    println!("[*] Adding order: {}", order3);
    trading_system.handle_order(&order3);

    let orders = trading_system.ask_orders.get(&100).unwrap();

    assert_eq!(orders.len(), 1);
    assert_eq!(orders[0].quantity, 10);
    assert_eq!(orders[0].owner, 2);
}

#[test]
fn test_buy_over_to_consume_first() {
    let mut trading_system = TradingSystem::new();
    let order1 = Order {
        order_type: OrderType::Ask,
        price: 100,
        quantity: 5,
        owner: 1,
    };
    let order2 = Order {
        order_type: OrderType::Ask,
        price: 100,
        quantity: 10,
        owner: 2,
    };
    let order3 = Order {
        order_type: OrderType::Bid,
        price: 100,
        quantity: 10,
        owner: 3,
    };
    println!("[*] Adding order: {}", order1);
    trading_system.handle_order(&order1);
    println!("[*] Adding order: {}", order2);
    trading_system.handle_order(&order2);
    println!("[*] Adding order: {}", order3);
    trading_system.handle_order(&order3);

    let orders = trading_system.ask_orders.get(&100).unwrap();

    assert_eq!(orders.len(), 1);
    assert_eq!(orders[0].quantity, 5);
    assert_eq!(orders[0].owner, 2);
}

#[test]
fn test_sell_not_enough() {
    let mut trading_system = TradingSystem::new();
    let order1 = Order {
        order_type: OrderType::Bid,
        price: 100,
        quantity: 5,
        owner: 1,
    };
    let order2 = Order {
        order_type: OrderType::Bid,
        price: 100,
        quantity: 10,
        owner: 2,
    };
    let order3 = Order {
        order_type: OrderType::Ask,
        price: 100,
        quantity: 3,
        owner: 3,
    };
    trading_system.handle_order(&order1);
    trading_system.handle_order(&order2);
    trading_system.handle_order(&order3);

    let orders = trading_system.bid_orders.get(&100).unwrap();

    assert_eq!(orders.len(), 2);
    assert_eq!(orders[0].quantity, 2);
    assert_eq!(orders[0].owner, 1);
    assert_eq!(orders[1].quantity, 10);
    assert_eq!(orders[1].owner, 2);
}
