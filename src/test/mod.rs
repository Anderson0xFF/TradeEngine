use serde_json::Value;

use crate::{book::Book, orders, trade_history::TradeHistory};

fn search_order_file(file: &str) -> Result<Vec<Value>, ()> {
    match orders::open_orders_file(file) {
        Ok(data) => {
            return Ok(data);
        }
        Err(_) => return Err(()),
    };
}

pub fn run(order_json_file: &str) {
    println!("Looking for {}", order_json_file);
    let data = search_order_file(order_json_file);

    assert_ne!(data, Err(()));
    println!("Looking for {} [OK]", order_json_file);

    let mut history = TradeHistory::new();
    let mut book = Book::new();

    println!("Reader: {}", order_json_file);
    book.reader_orders(data.unwrap());

    println!("Reader: {} OK", order_json_file);
    println!("Save Orderbook");

    book.save("orderbook.json");
    println!("Save Orderbook: [OK]");

    println!("Processing trades:");
    book.process_orders(&mut history);

    println!("Processing trades:[OK]");
    history.save("trades.json");

    println!("Save trades historic in : {}", "trades.json");

}