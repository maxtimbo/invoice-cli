use std::fmt;
use serde::{Serialize, Deserialize};
use crate::models::contact::Contact;

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub id: i64,
    pub name: String,
    pub logo: Option<Vec<u8>>,
    pub contact: Contact,
}

impl fmt::Display for Company {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID:\t\t{}\n", self.id)?;
        write!(f, "Name:\t\t{}\n", self.name)?;
        write!(f, "Has Logo:\t{}\n", self.logo.is_some())?;
        write!(f, "Contact Information:\n{}", self.contact)
    }
}
