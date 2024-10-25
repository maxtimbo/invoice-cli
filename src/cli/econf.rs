use anyhow::Result;
use clap::Args;

use crate::db::InvoiceDB;

#[derive(Debug, Args, PartialEq)]
pub struct EmailConfig {
    smtp_server: String,
    port: i64,
    tls: bool,
    username: String,
    password: String,
    fromname: String,
    fromemail: String,
    signature: String,
}

impl EmailConfig {
    pub fn new(&self, db: &InvoiceDB) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

