#![allow(dead_code)]
use serde_json::Value;

use crate::{
    orders::{orderside::OrderSide, ordertype::OrderType, Order},
    trade_history::{OrderStatus, TradeHistory},
};

pub struct Book {
    orders: Vec<Order>,
}

impl Book {
    pub fn new() -> Self {
        Self { orders: Vec::new() }
    }

    fn decrement_amount(&mut self, order_id: u32, amount: f32) {
        match self.find_order_by_id_as_mut(order_id) {
            Some((order, _)) => {
                let amount = order.amount() - amount;
                order.set_amount(amount);
            },
            None => return,
        }
    }

    pub fn find_order_by_id_as_mut(&mut self, order_id: u32) -> Option<(&mut Order, usize)> {
        let mut index = 0;
        for value in &mut self.orders {
            if value.order_id() == order_id {
                return Some((&mut self.orders[index], index));
            }
            index += 1;
        }
        None
    }

    fn buy_all(&self) -> Vec<Order> {
        let mut buffer = Vec::new();
        for value in self.orders.clone() {
            if value.side().eq(&OrderSide::BUY) {
                buffer.push(value);
            }
        }
        buffer
    }

    fn sell_all(&self) -> Vec<Order> {
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
                Ok(order) => match order.type_op() {
                    OrderType::CREATE => self.orders.push(order),
                    OrderType::DELETE => self.remove(order.order_id()),
                },
                Err(e) => println!("ERROR: Book ID:{} {}", i, e),
            };
        }
    }
    
    pub fn find_order_by_id(&self, order_id: u32) -> Option<(&Order, usize)> {
        let mut index = 0;
        for value in &self.orders {
            if value.order_id() == order_id {
                return Some((&self.orders[index], index));
            }
            index += 1;
        }
        None
    }

    pub fn remove(&mut self, order_id: u32) {
        match self.find_order_by_id(order_id) {
            Some((_, index)) => self.orders.remove(index),
            None => todo!(),
        };
    }

    pub fn process_orders(&mut self, tradehistory: &mut TradeHistory) {
        for buy in self.buy_orders_sort() {
            for sell in self.sell_orders_sort() {
                if (buy.limit_price() >= sell.limit_price())
                    && (buy.account_id() != sell.account_id())
                {
                    if buy.amount() > sell.amount() {
                        self.decrement_amount(buy.order_id(), sell.amount());
                        tradehistory.add_history(OrderStatus::PARTIAL, (sell.clone(), buy.clone()));
                        self.remove(sell.order_id());
                    } else if sell.amount() > buy.amount() {
                        self.decrement_amount(sell.order_id(), buy.amount());
                        tradehistory.add_history(OrderStatus::PARTIAL, (sell.clone(), buy.clone()));
                        self.remove(buy.order_id());
                    } else {
                        tradehistory.add_history(OrderStatus::COMPLETE, (sell.clone(), buy.clone()));
                        self.remove(buy.order_id());
                        self.remove(sell.order_id());
                        break;
                    }
                }
            }
        }
    }
}
