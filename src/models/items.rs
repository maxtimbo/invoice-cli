use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl fmt::Display for Items {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (Rate: {})", self.name, self.rate)
    }
}

impl PartialEq for Items {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Items {}

impl Hash for Items {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

