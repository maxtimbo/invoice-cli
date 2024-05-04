use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Methods {
    pub id: i64,
    pub name: String,
}

impl fmt::Display for Methods {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {} - Name: {}", self.id, self.name)
    }
}

impl Methods {
    pub fn display(&self) {
        println!(
            "Payment Method\n\
            ~~~~~~~~~~~~\n\
            id:\t\t{}\n\
            name:\t\t{}",
            self.id, self.name,
        );
    }
}
