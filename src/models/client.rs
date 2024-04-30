use std::fmt;
use crate::models::contact::Contact;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: i64,
    pub name: String,
    pub contact: Contact,
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID:\t\t{}\n", self.id)?;
        write!(f, "Name:\t\t{}\n", self.name)?;
        write!(f, "Contact Information:\n{}", self.contact)
    }
}

impl Client {
    pub fn display(&self) {
        println!("Client\n\
            ~~~~~~~~~~~~\n\
            id:\t\t{}\n\
            name:\t\t{}\n\
            phone:\t\t{}\n\
            email:\t\t{}\n\
            addr1:\t\t{}\n\
            addr2:\t\t{}\n\
            city:\t\t{}\n\
            state:\t\t{}\n\
            zip:\t\t{}",
            self.id,
            self.name,
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
            

