use crate::models::contact::Contact;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub id: i64,
    pub name: String,
    pub logo: Option<Vec<u8>>,
    pub contact: Contact,
}

impl Company {
    pub fn display(&self) {
        println!("Company\n\
            ~~~~~~~~~~~~\n\
            id:\t\t{}\n\
            name:\t\t{}\n\
            logo:\t\t\n\
            phone:\t\t{}\n\
            email:\t\t{}\n\
            addr1:\t\t{}\n\
            addr2:\t\t{}\n\
            city:\t\t{}\n\
            state:\t\t{}\n\
            zip:\t\t{}",
            self.id,
            self.name,
            //self.logo.as_deref().unwrap_or("None"),
            self.contact.phone.as_deref().unwrap_or("None"),
            self.contact.email.as_deref().unwrap_or("None"),
            self.contact.addr1.as_deref().unwrap_or("None"),
            self.contact.addr2.as_deref().unwrap_or("None"),
            self.contact.city.as_deref().unwrap_or("None"),
            self.contact.state.as_deref().unwrap_or("None"),
            self.contact.zip.as_deref().unwrap_or("None")
        );
    }
}

