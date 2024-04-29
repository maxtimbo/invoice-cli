use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Methods {
    pub id: i64,
    pub name: String,
}

impl Methods {
    pub fn display(&self) {
        println!("Payment Method\n\
            ~~~~~~~~~~~~\n\
            id:\t\t{}\n\
            name:\t\t{}",
            self.id,
            self.name,
        );
    }
}

