use std::fmt;

use serde::{Deserialize, Serialize};

use crate::db::prepare::ModelActions;

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub phone: Option<String>,
    pub email: Option<String>,
    pub addr1: Option<String>,
    pub addr2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}

impl Contact {
    pub fn default() -> Self {
        Self {
            phone: None,
            email: None,
            addr1: None,
            addr2: None,
            city: None,
            state: None,
            zip: None,
        }
    }
}

impl ModelActions for Contact {
    fn fields(&self) -> Vec<String> {
        let mut fnames = Vec::new();
        if self.phone.is_some() {
            fnames.push("phone".to_string());
        }
        if self.email.is_some() {
            fnames.push("email".to_string());
        }
        if self.addr1.is_some() {
            fnames.push("addr1".to_string());
        }
        if self.addr2.is_some() {
            fnames.push("addr2".to_string());
        }
        if self.city.is_some() {
            fnames.push("city".to_string());
        }
        if self.state.is_some() {
            fnames.push("state".to_string());
        }
        if self.zip.is_some() {
            fnames.push("zip".to_string());
        }
        fnames
    }
    fn values(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<rusqlite::types::Value> = Vec::new();
        if self.phone.is_some() {
            values.push(self.phone.clone().into());
        }
        if self.email.is_some() {
            values.push(self.email.clone().into());
        }
        if self.addr1.is_some() {
            values.push(self.addr1.clone().into());
        }
        if self.addr2.is_some() {
            values.push(self.addr2.clone().into());
        }
        if self.city.is_some() {
            values.push(self.city.clone().into());
        }
        if self.state.is_some() {
            values.push(self.state.clone().into());
        }
        if self.zip.is_some() {
            values.push(self.zip.clone().into());
        }
        values
    }
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref phone) = self.phone {
            write!(f, "Phone:\t\t{}\n", phone)?;
        }
        if let Some(ref email) = self.email {
            write!(f, "Email:\t\t{}\n", email)?;
        }
        if let Some(ref addr1) = self.addr1 {
            write!(f, "Addr1:\t\t{}\n", addr1)?;
        }
        if let Some(ref addr2) = self.addr2 {
            write!(f, "Addr2:\t\t{}\n", addr2)?;
        }
        if let Some(ref city) = self.city {
            write!(f, "City:\t\t{}\n", city)?;
        }
        if let Some(ref state) = self.state {
            write!(f, "State:\t\t{}\n", state)?;
        }
        if let Some(ref zip) = self.zip {
            write!(f, "Zip:\t\t{}\n", zip)?;
        }
        Ok(())
    }
}
