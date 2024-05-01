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
