use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Items {
    pub id: i64,
    pub name: String,
    pub rate: i32,
}

impl Items {
    pub fn display(&self) {
        println!("Item\n\
            ~~~~~~~~~~~~\n\
            id:\t\t{}\n\
            name:\t\t{}\n\
            rate:\t\t{}",
            self.id,
            self.name,
            self.rate
        );
    }
}

