use crate::book::Book;
mod book;
mod orders;

fn main() {
    let data = orders::open_orders_file("../orders.json");

    let data = match data {
        Ok(data) => data,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let mut book = Book::new();
    book.reader_orders(data);

    let orders = book.sell_orders_sort();
    for value in orders {
        println!("{:#?}", value);
    }
}
