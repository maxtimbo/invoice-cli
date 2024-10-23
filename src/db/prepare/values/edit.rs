use rusqlite::types::Value;

use crate::db::prepare::PrepValues;
use crate::cli::edit::*;
use crate::models::invoice::{InvoiceStage, PaidStatus};
use crate::validators::{ValidSize, ValidImage};

use invoice_cli::decimal_to_i64;


impl PrepValues for EditCompany {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        if self.name.is_some() {
            values.push(self.name.clone().into());
        }
        if let Some(logo) = &self.logo {
            if self.is_valid_image(&logo) {
                match self.read_image(&logo) {
                    Ok(data) => values.push(Value::Blob(data)),
                    Err(e) => eprintln!("Error reading image file: {}", e),
                }
            } else {
                eprintln!("Invalid image file type.");
            }
        }
        values.extend(self.contact.values());
        values
    }
}

impl PrepValues for EditClient {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        if self.name.is_some() {
            values.push(self.name.clone().into());
        }
        values.extend(self.contact.values());
        values
    }
}

impl PrepValues for EditTerms {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        if self.name.is_some() {
            values.push(self.name.clone().into());
        }
        if self.due.is_some() {
            values.push(self.due.into());
        }
        values
    }
}

impl PrepValues for EditMethod {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        if self.name.is_some() {
            values.push(self.name.clone().into());
        }
        if self.link.is_some() {
            values.push(self.link.clone().into());
        }
        if let Some(qr) = &self.qr {
            if self.is_valid_image(&qr) {
                match self.read_image(&qr) {
                    Ok(data) => values.push(Value::Blob(data)),
                    Err(e) => eprintln!("Error reading image file: {}", e),
                }
            } else {
                eprintln!("Invalid image file type.");
            }
        }
        values
    }
}

impl PrepValues for EditItem {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        if self.name.is_some() {
            values.push(self.name.clone().into());
        }
        if let Some(rate) = self.rate {
            values.push(decimal_to_i64!(rate).into());
        }
        values
    }
}

impl PrepValues for EditTemplate {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        if self.name.is_some() {
            values.push(self.name.clone().into());
        }
        if self.company.is_some() {
            values.push(self.company.into());
        }
        values
    }
}

impl PrepValues for EditInvoice {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        if self.show_methods.is_some() {
            values.push(self.show_methods.into());
        }
        if self.show_notes.is_some() {
            values.push(self.show_notes.into());
        }
        if let Some(stage) = &self.stage {
            match stage {
                InvoiceStage::Quote => { values.push("Quote".to_string().into()) }
                InvoiceStage::Invoice => { values.push("Invoice".to_string().into()) }
            }
        }
        if let Some(status) = &self.status {
            match status {
                PaidStatus::Waiting => { values.push("Waiting".to_string().into()) }
                PaidStatus::Paid { date, check } => {
                    values.push("Paid".to_string().into());
                    values.push(date.to_string().into());
                    if let Some(check_str) = check {
                        values.push(check_str.to_string().into());
                    } else {
                        values.push("None".to_string().into());
                    }
                }
                PaidStatus::Failed { date } => {
                    values.push("Failed".to_string().into());
                    values.push(date.to_string().into());
                }
                PaidStatus::Refunded { date } => {
                    values.push("Refunded".to_string().into());
                    values.push(date.to_string().into());
                }
            }
        }
        if let Some(notes) = &self.notes {
            values.push(notes.to_string().into());
        }
            
        values
    }
}
