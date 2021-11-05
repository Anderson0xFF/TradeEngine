#![allow(dead_code)]
use serde_json::Value;

use crate::orders::{orderside::OrderSide, Order};

pub struct Book {
    orders: Vec<Order>,
}

impl Book {
    pub fn new() -> Self {
        Self { orders: Vec::new() }
    }

    pub fn buy_all(&self) -> Vec<Order> {
        let mut buffer = Vec::new();
        for value in self.orders.clone() {
            if value.side().eq(&OrderSide::BUY) {
                buffer.push(value);
            }
        }
        buffer
    }

    pub fn sell_all(&self) -> Vec<Order> {
        let mut buffer = Vec::new();
        for value in self.orders.clone() {
            if value.side().eq(&OrderSide::SELL) {
                buffer.push(value);
            }
        }

        buffer
    }

    pub fn sell_orders_sort(&self) -> Vec<Order> {
        let mut buffer = self.sell_all();
        buffer.sort_by(|x, y| x.limit_price().partial_cmp(&y.limit_price()).unwrap());
        buffer
    }

    pub fn buy_orders_sort(&self) -> Vec<Order> {
        let mut buffer = self.buy_all();
        buffer.sort_by(|x, y| x.limit_price().partial_cmp(&y.limit_price()).unwrap());
        buffer.reverse();
        buffer
    }

    pub fn reader_orders(&mut self, orders: Vec<Value>) {
        for i in 0..orders.len() {
            match Order::from_value(&orders[i]) {
                Ok(order) => self.orders.push(order),
                Err(e) => println!("Book ID:{} {}", i, e),
            };
        }
    }

    pub fn process_orders(&mut self) {
        println!("OrdersBook:");
        for value in &self.orders {
            println!("{:#?}", value);
        }
    }
}
