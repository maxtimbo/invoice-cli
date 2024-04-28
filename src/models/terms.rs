#[derive(Debug)]
pub struct Terms {
    pub id: i32,
    pub name: String,
    pub due: i32,
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

