use crate::{book::Book, orders, trade_history::TradeHistory};
use serde_json::Value;

const ORDERS_PATH: &str = "orders.json";
const ORDERBOOK_PATH: &str = "orderbook.json";
const TRADE_HISTORIC_PATH: &str = "trades.json";

fn search_order_file(file: &str) -> Result<Vec<Value>, ()> {
    match orders::open_orders_file(file) {
        Ok(data) => {
            return Ok(data);
        }
        Err(_) => return Err(()),
    };
}

pub fn run() {
    println!("Looking for {}", ORDERS_PATH);
    let data = search_order_file(ORDERS_PATH);

    assert_ne!(data, Err(()));
    println!("Looking for {} [OK]", ORDERS_PATH);

    let mut history = TradeHistory::new();
    let mut book = Book::new();

    println!("Reader: {}", ORDERS_PATH);
    book.reader_orders(data.unwrap());

    println!("Reader: {} OK", ORDERS_PATH);
    println!("Save Orderbook");

    book.save(ORDERBOOK_PATH);
    println!("Save Orderbook: [OK]");

    println!("Processing trades:");
    book.process_orders(&mut history);

    println!("Processing trades:[OK]");
    history.save(TRADE_HISTORIC_PATH);

    println!("Save trades historic in : {}", "trades.json");
}
