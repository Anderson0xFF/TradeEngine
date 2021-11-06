#![allow(dead_code)]

use crate::orders::Order;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OrderStatus {
    COMPLETE,
    PARTIAL,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct History {
    status: OrderStatus,
    time: String,
    pair: (Order, Order),
}

impl History {
    fn new(status: OrderStatus, time: String, pair: (Order, Order)) -> Self {
        Self { status, time, pair }
    }
}
pub struct TradeHistory {
    history: Vec<History>,
}

impl TradeHistory {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn add_history(&mut self, status: OrderStatus, pair: (Order, Order)) {
        let time: DateTime<Local> = Local::now();
        let history = History::new(status, time.to_string(), pair);
        self.history.push(history);
    }

    pub fn save(&mut self) {
        for value in &self.history {
            let va = serde_json::to_string_pretty(&value).unwrap();
            println!("{}", va);
        }
    }

    /// Get a reference to the trade history's history.
    fn history(&self) -> &[History] {
        self.history.as_ref()
    }
}
