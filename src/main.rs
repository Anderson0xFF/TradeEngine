use crate::{book::Book, tradehistory::TradeHistory};
mod book;
mod orders;
mod tradehistory;
fn main() {
    let data = orders::open_orders_file("orders.json");

    let data = match data {
        Ok(data) => data,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let mut history = TradeHistory::new();
    let mut book = Book::new();
    book.reader_orders(data);
    book.process_orders(&mut history);
    history.save();
}
