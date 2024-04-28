#[derive(Debug)]
pub struct Items {
    pub id: i32,
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

