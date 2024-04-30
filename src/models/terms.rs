use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Terms {
    pub id: i64,
    pub name: String,
    pub due: i64,
}

impl Terms {
    pub fn display(&self) {
        println!("Terms\n\
            ~~~~~~~~~~~~\n\
            id:\t\t{}\n\
            name:\t\t{}\n\
            due:\t\t{}",
            self.id,
            self.name,
            self.due
        );
    }
}

