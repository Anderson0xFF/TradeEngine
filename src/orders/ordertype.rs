use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
pub enum OrderType {
    CREATE,
    DELETE,
}

impl OrderType {
    pub fn from_str(input: &str) -> Result<OrderType, &str> {
        match input {
            "CREATE" => Ok(OrderType::CREATE),
            "DELETE" => Ok(OrderType::DELETE),
            _ => Err("order type not supported."),
        }
    }
}
