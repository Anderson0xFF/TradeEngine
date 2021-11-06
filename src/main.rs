mod book;
mod orders;
mod trade_history;
mod test;
const ORDER_FILE: &str = "orders.json";
fn main() {
    test::run(ORDER_FILE);
}
