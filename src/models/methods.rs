#[derive(Debug)]
pub struct Methods {
    pub id: i32,
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

