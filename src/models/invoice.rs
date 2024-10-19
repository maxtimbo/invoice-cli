use std::collections::HashMap;
use std::fmt;

use chrono::{Duration, NaiveDate};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use rust_decimal::Decimal;

use crate::models::EntityDeleter;
use crate::models::client::Client;
use crate::models::company::Company;
use crate::models::items::Items;
use crate::models::methods::Methods;
use crate::models::terms::Terms;

use crate::cli::delete::{DeleteTemplate, DeleteInvoice};

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub id: i64,
    pub name: String,
    pub company: Company,
    pub client: Client,
    pub terms: Terms,
    pub methods: Vec<Methods>,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID:\t\t{}\n", self.id)?;
        write!(f, "Name:\t\t{}\n\n", self.name)?;
        write!(f, "Company Information:\n{}\n", self.company)?;
        write!(f, "Client Information:\n{}\n", self.client)?;
        write!(f, "Template Terms:\n")?;
        write!(f, "{}\n", self.terms)?;
        write!(f, "Payment Status:\n")?;
        write!(f, "Template Payment Methods:\n")?;
        for method in &self.methods {
            write!(f, "{}\n", method)?;
        }
        Ok(())
    }
}

impl EntityDeleter<Template> for Template {
    type Output = DeleteTemplate;
    fn delete(&self) -> Result<Self::Output, anyhow::Error> {
        Ok(DeleteTemplate { id: self.id })
    }
}

#[derive(Debug)]
pub struct Invoice {
    pub id: i64,
    pub template: Template,
    pub attributes: InvoiceAttrs,
    pub date: String,
    pub notes: Option<String>,
    pub items: HashMap<Items, i64>,
}

#[derive(Debug, PartialEq)]
pub struct InvoiceAttrs {
    pub show_methods: bool,
    pub show_notes: bool,
    pub stage: InvoiceStage,
    pub status: PaidStatus,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum InvoiceStage {
    Quote,
    Invoice,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum PaidStatus {
    Waiting,
    Paid { date: String, check: Option<String> },
    Failed { date: String},
    Refunded { date: String},
}

impl Invoice {
    fn calculate_subtotals(&self) -> Vec<ItemDetail> {
        let mut item_details: Vec<ItemDetail> = self.items
            .iter()
            .map(|(item, &quantity)| ItemDetail {
                name: item.name.clone(),
                rate: item.rate,
                quantity,
                subtotal: item.rate * Decimal::from(quantity),
            })
            .collect();
        item_details.sort_by(|a, b| a.name.cmp(&b.name));
        item_details
    }
    fn calculate_total(&self) -> Decimal {
        self.calculate_subtotals().iter().map(|item| item.subtotal).sum()
    }
    fn issue_date(&self) -> NaiveDate {
        NaiveDate::parse_from_str(&self.date, "%Y%m%d").unwrap()
    }
    fn due_date(&self) -> NaiveDate {
        self.issue_date() + Duration::days(self.template.terms.due)
    }
}

impl fmt::Display for Invoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID:\t\t{}\n", self.id)?;
        write!(f, "Date:\t\t{}\n\n", self.date)?;
        write!(f, "Template Information:\n{}\n", self.template)?;
        match self.attributes.stage {
            InvoiceStage::Quote => {
                write!(f, "Stage:\t\tQuote\n")?;
            },
            InvoiceStage::Invoice => {
                write!(f, "Stage:\t\tInvoice\n")?;
            }
        }
        match &self.attributes.status {
            PaidStatus::Waiting => {
                write!(f, "Waiting for payment")?;
            },
            PaidStatus::Paid { date, check } => {
                write!(f, "Paid - Date: {}, Check number: {}", date, check.is_some())?; 
            },
            PaidStatus::Failed { date } => {
                write!(f, "Failed - Date: {}", date)?;
            },
            PaidStatus::Refunded { date } => {
                write!(f, "Refunded - Date: {}", date)?;
            }
        }
        write!(f, "Notes:\n{}\n", self.notes.is_some())?;
        write!(f, "Invoice Items:\n")?;
        write!(f, "Item\t\t\t| Rate\t| Quantity\t| Subtotal\n")?;
        for item in &self.calculate_subtotals() {
            write!(f, "{}\t| {}\t| ${}\t\t| ${}\n",
                        item.name,
                        item.quantity,
                        item.rate,
                        item.subtotal)?;
        }
        write!(f, "\t\t\t\t\tTotal:\t  ${}\n", &self.calculate_total())?;
        write!(f, "Due Date: {}", &self.due_date().format("%B %d, %Y").to_string())?;
        Ok(())
    }
}

impl EntityDeleter<Invoice> for Invoice {
    type Output = DeleteInvoice;
    fn delete(&self) -> Result<Self::Output, anyhow::Error> {
        Ok(DeleteInvoice { id: self.id })
    }
}

impl Serialize for Invoice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Invoice", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("template", &self.template)?;
        state.serialize_field("date", &self.issue_date().format("%B %d, %Y").to_string())?;
        state.serialize_field("items", &self.calculate_subtotals())?;
        state.serialize_field("total", &self.calculate_total())?;
        state.serialize_field("due_date", &self.due_date().format("%B %d, %Y").to_string())?;
        state.end()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InvoiceItem {
    pub item: i64,
    pub quantity: i64,
}

#[derive(Debug, Serialize)]
struct ItemDetail {
    name: String,
    rate: Decimal,
    quantity: i64,
    subtotal: Decimal,
}
