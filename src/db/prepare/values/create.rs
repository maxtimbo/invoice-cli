use rusqlite::types::Value;
use serde_json;

use crate::db::prepare::PrepValues;
use crate::cli::create::*;
use crate::models::invoice::{InvoiceStage, PaidStatus};
use crate::validators::{ValidSize, ValidImage};

use invoice_cli::decimal_to_i64;

impl PrepValues for CreateCompany {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        values.push(self.name.clone().into());
        if let Some(logo) = &self.logo {
            if self.is_valid_image(logo) {
                match self.read_image(logo) {
                    Ok(data) => values.push(Value::Blob(data)),
                    Err(e) => eprintln!("Error reading image file: {}", e),
                }
            } else {
                eprintln!("Invalid image file type.");
            }
        }
        if let Some(contact) = &self.contact {
            values.extend(contact.values());
        }
        values
    }
}

impl PrepValues for CreateClient {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        values.push(self.name.clone().into());
        if let Some(contact) = &self.contact {
            values.extend(contact.values());
        }
        values
    }
}

impl PrepValues for CreateTerms {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        values.push(self.name.clone().into());
        values.push(self.due.into());
        values
    }
}

impl PrepValues for CreateMethod {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        values.push(self.name.clone().into());
        if self.link.is_some() {
            values.push(self.link.clone().into());
        }

        if let Some(qr) = &self.qr{
            if self.is_valid_image(qr) {
                match self.read_image(qr) {
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

impl PrepValues for CreateItem {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        values.push(self.name.clone().into());
        values.push(decimal_to_i64!(self.rate).into());
        values
    }
}

impl PrepValues for CreateTemplate {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        values.push(self.name.clone().into());
        values.push(self.company.into());
        values.push(self.client.into());
        values.push(self.terms.into());
        let methods_json =
            serde_json::to_string(&self.methods).expect("Failed to serialize to JSON");
        println!("{:?}", methods_json);
        values.push(methods_json.into());
        values
    }
}

impl PrepValues for CreateInvoice {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        values.push(self.template.into());
        let date_str = self.date.format("%Y%m%d").to_string();
        values.push(Value::from(date_str));
        values.push(self.attributes.show_methods.into());
        values.push(self.attributes.show_notes.into());

        let stage_str = match self.attributes.stage {
            InvoiceStage::Quote => "Quote".to_string(),
            InvoiceStage::Invoice => "Invoice".to_string(),
        };
        values.push(Value::from(stage_str));

        let status_str = match &self.attributes.status {
            PaidStatus::Waiting => "Waiting".to_string(),
            PaidStatus::PastDue => "Past Due".to_string(),
            PaidStatus::Paid { .. } => "Paid".to_string(),
            PaidStatus::Failed { .. } => "Failed".to_string(),
            PaidStatus::Refunded { .. } => "Refunded".to_string(),
        };
        values.push(Value::from(status_str));

        let status_date = match &self.attributes.status {
            PaidStatus::Paid { date, .. }
            | PaidStatus::Failed { date }
            | PaidStatus::Refunded { date } => Some(date.clone()),
            _ => None,
        };
        values.push(status_date.unwrap_or_default().into());

        let status_check = match &self.attributes.status {
            PaidStatus::Paid { check, .. } => check.clone(),
            _ => None,
        };
        values.push(status_check.unwrap_or_default().into());

        values.push(self.notes.clone().into());
        let items_json = serde_json::to_string(&self.items).expect("Failed to serialize to JSON");
        values.push(items_json.into());
        values
    }
}
