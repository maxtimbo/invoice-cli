use crate::db::prepare::{PrepFields, PrepValues};
use clap::Args;
use serde::Deserialize;

#[derive(Debug, Args, Deserialize)]
pub struct Contact {
    #[arg(long, short)]
    pub phone: Option<String>,

    #[arg(long, short)]
    pub email: Option<String>,

    #[arg(long, short)]
    pub addr1: Option<String>,

    #[arg(long)]
    pub addr2: Option<String>,

    #[arg(long, short)]
    pub city: Option<String>,

    #[arg(long, short)]
    pub state: Option<String>,

    #[arg(long, short)]
    pub zip: Option<String>,
}

impl PrepFields for Contact {
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
}

impl PrepValues for Contact {
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
