use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
pub enum OrderSide {
    BUY,
    SELL,
}

impl OrderSide {
    pub fn from_str(input: &str) -> Result<OrderSide, &str> {
        match input {
            "BUY" => Ok(OrderSide::BUY),
            "SELL" => Ok(OrderSide::SELL),
            _ => Err("order side not supported."),
        }
    }
}
