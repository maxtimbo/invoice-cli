use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use chrono::{Duration, NaiveDate};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use rust_decimal::Decimal;
use inquire::{MultiSelect, InquireError, Confirm, Select, DateSelect};
use pulldown_cmark::{html, Parser};

use crate::models::{prompt_optional, editor_optional};
use crate::models::{EntityDeleter, EntityUpdater};
use crate::models::items::Items;
use crate::models::template::Template;

use crate::cli::delete::DeleteInvoice;
use crate::cli::edit::EditInvoice;

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

impl FromStr for InvoiceStage {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Quote" => Ok(InvoiceStage::Quote),
            "Invoice" => Ok(InvoiceStage::Invoice),
            _ => Err(format!("Invalid InvoiceStage: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum PaidStatus {
    Waiting,
    PastDue,
    Paid { date: String, check: Option<String> },
    Failed { date: String},
    Refunded { date: String},
}

impl FromStr for PaidStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Waiting" => Ok(PaidStatus::Waiting),
            "Past Due" => Ok(PaidStatus::PastDue),
            "Paid" => {
                let date = DateSelect::new("Select payment date").prompt().unwrap();
                let check = prompt_optional("Enter check number if applicable or enter 'None':", "").unwrap();
                Ok(PaidStatus::Paid { date: date.format("%Y%m%d").to_string(), check })
            }
            "Failed" => {
                let date = DateSelect::new("Select failed payment date").prompt().unwrap();
                Ok(PaidStatus::Failed { date: date.format("%Y%m%d").to_string() })
            }
            "Refunded" => {
                let date = DateSelect::new("Select refunded payment date").prompt().unwrap();
                Ok(PaidStatus::Refunded { date: date.format("%Y%m%d").to_string() })
            }
            _ => Err(format!("Invalid PaidStatus: {}", s)),
        }
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

        write!(f, "Payment status:\t")?;
        match &self.attributes.status {
            PaidStatus::Waiting => {
                write!(f, "Waiting for payment\n")?;
            },
            PaidStatus::PastDue => {
                write!(f, "Payment is past due\n")?;
            }
            PaidStatus::Paid { date, check } => {
                write!(f, "Paid\nDate:\t\t{}\n", date)?; 
                if let Some(check_str) = check {
                    write!(f, "Check:\t\t{}\n\n", check_str)?;
                }
            },
            PaidStatus::Failed { date } => {
                write!(f, "Failed\nDate:\t\t{}\n", date)?;
            },
            PaidStatus::Refunded { date } => {
                write!(f, "Refunded\nDate:\t\t{}\n", date)?;
            }
        }
        if let Some(notes) = &self.notes {
            write!(f, "Notes:\n{}\n\n", notes.to_string())?;
        }

        write!(f, "Invoice attributes:\n")?;
        write!(f, "Show notes:\t\t{}\n", self.attributes.show_notes)?;
        write!(f, "Show payment methods:\t{}\n\n", self.attributes.show_methods)?;

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
        state.serialize_field("show_methods", &self.attributes.show_methods)?;
        state.serialize_field("show_notes", &self.attributes.show_notes)?;
        let stage_str = match &self.attributes.stage {
            InvoiceStage::Quote => "Quote",
            InvoiceStage::Invoice => "Invoice",
        };
        state.serialize_field("invoice_stage", &stage_str)?;
        if let Some(notes) = &self.notes {
            let parser = Parser::new(notes);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
            state.serialize_field("notes", &html_output)?;
        }
        state.end()
    }
}

impl EntityUpdater<Invoice> for Invoice {
    type Output = EditInvoice;
    fn update(&self) -> Result<Self::Output, InquireError> {
        println!("{}", self);
        let fields = vec!["show methods", "show notes", "invoice stage", "payment status", "notes"];
        let mut edit_invoice = EditInvoice {
            id: self.id,
            show_methods: None,
            show_notes: None,
            stage: None,
            status: None,
            notes: None,
        };
        let selected_fields = MultiSelect::new("Select fields to update:", fields).prompt()?;
        for field in selected_fields {
            match field {
                "show methods" => {
                    let show_methods = Confirm::new("Show payment methods?")
                        .with_default(self.attributes.show_methods)
                        .prompt()?;
                    edit_invoice.show_methods = Some(show_methods);
                }
                "show notes" => {
                    let show_notes = Confirm::new("Show notes?")
                        .with_default(self.attributes.show_notes)
                        .prompt()?;
                    edit_invoice.show_notes = Some(show_notes);
                }
                "invoice stage" => {
                    let stage_select = Select::new("Select invoice stage:", vec!["Quote", "Invoice"]).prompt()?;
                    let stage = InvoiceStage::from_str(&stage_select)
                        .map_err(|err| InquireError::Custom(err.to_string().into()))?;
                    edit_invoice.stage = Some(stage);
                }
                "payment status" => {
                    let statuses = vec!["Waiting", "Paid", "Failed", "Refunded"];
                    let selected_status = Select::new("Select payment status:", statuses).prompt()?;
                    let status = PaidStatus::from_str(&selected_status)
                        .map_err(|err| InquireError::Custom(err.to_string().into()))?;
                    edit_invoice.status = Some(status)
                }
                "notes" => {
                    let notes = editor_optional("Enter new notes, or write 'None' to clear", &self.notes.clone().unwrap_or_default())?;
                    edit_invoice.notes = notes;
                }
                _ => {}
            }
        }
        Ok(edit_invoice)
    }
}
