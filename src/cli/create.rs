use std::path::PathBuf;

use chrono::NaiveDate;
use clap::{Args, Subcommand};
use rusqlite::types::Value;
use serde::Deserialize;
use serde_json;
use rust_decimal::Decimal;

use crate::db::InvoiceDB;
use crate::cli::contact::Contact;
use crate::db::prepare::{PrepCreate, PrepUpdate, PrepFields, PrepValues, TableName};
use crate::models::EntityUpdater;
use crate::models::invoice::{InvoiceItem, InvoiceAttrs, InvoiceStage, PaidStatus};
use crate::validators::{ValidImage, ValidSize};
use invoice_cli::decimal_to_i64;

#[derive(Subcommand, Debug, PartialEq)]
pub enum CreateCommands {
    /// Supply a json file for entity creation.
    /// Reference the example.json file to see the file structure.
    FromJson(FromJSON),
    /// Create a company
    Company { name: String },
    /// Create a client
    Client { name: String },
    /// Create payment terms
    Terms(CreateTerms),
    /// Create payment methods
    Method { name: String },
    /// Create inventory items
    Item(CreateItem),
}

pub fn handle_create(create: &CreateCommands, db: &InvoiceDB) -> Result<(), anyhow::Error> {
    match create {
        CreateCommands::FromJson(obj) => match FromJSON::from(obj) {
            Ok(entities) => {
                if let Some(companies) = entities.company {
                    for company in companies {
                        db.create_entry(company.prepare())?;
                    }
                }
                if let Some(clients) = entities.client {
                    for client in clients {
                        db.create_entry(client.prepare())?;
                    }
                }
                if let Some(terms) = entities.terms {
                    for term in terms {
                        db.create_entry(term.prepare())?;
                    }
                }
                if let Some(methods) = entities.method {
                    for method in methods {
                        db.create_entry(method.prepare())?;
                    }
                }
                if let Some(items) = entities.item {
                    for item in items {
                        db.create_entry(item.prepare())?;
                    }
                }
            }
            Err(e) => eprintln!("Failed to parse JSON: {}", e),
        },
        CreateCommands::Company{ name: obj } => {
            let create_company = CreateCompany { name: obj.to_string(), logo: None, contact: None };
            let id = db.create_entry(create_company.prepare())?;
            let entity = db.get_company(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        }
        CreateCommands::Client { name: obj } => {
            let create_client = CreateClient { name: obj.to_string(), contact: None };
            let id = db.create_entry(create_client.prepare())?;
            let entity = db.get_client(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        }
        CreateCommands::Terms(obj) => {
            db.create_entry(CreateTerms::prepare(obj))?;
        }
        CreateCommands::Method { name: obj } => {
            let create_method = CreateMethod { name: obj.to_string(), link: None, qr: None };
            let id = db.create_entry(create_method.prepare())?;
            let entity = db.get_method(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        }
        CreateCommands::Item(obj) => {
            db.create_entry(CreateItem::prepare(obj))?;
        }
    }
    Ok(())
}

#[derive(Debug, Args, PartialEq)]
pub struct FromJSON {
    pub json_input: PathBuf,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CreateCompany {
    pub name: String,
    pub logo: Option<PathBuf>,
    pub contact: Option<Contact>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CreateClient {
    pub name: String,
    pub contact: Option<Contact>,
}

#[derive(Debug, Args, Deserialize, PartialEq)]
pub struct CreateTerms {
    pub name: String,
    #[arg(long, short)]
    pub due: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CreateMethod {
    pub name: String,
    pub link: Option<String>,
    pub qr: Option<PathBuf>,
}

#[derive(Debug, Args, Deserialize, PartialEq)]
pub struct CreateItem {
    pub name: String,
    #[arg(long, short)]
    pub rate: Decimal,
}

#[derive(Debug, PartialEq)]
pub struct CreateTemplate {
    pub name: String,
    pub company: i64,
    pub client: i64,
    pub terms: i64,
    pub methods: Vec<i64>,
}

#[derive(Debug, PartialEq)]
pub struct CreateInvoice {
    pub template: i64,
    pub date: NaiveDate,
    pub attributes: InvoiceAttrs,
    pub notes: Option<String>,
    pub items: Vec<InvoiceItem>,
}

impl PrepCreate for CreateCompany {}
impl PrepCreate for CreateClient {}
impl PrepCreate for CreateTerms {}
impl PrepCreate for CreateMethod {}
impl PrepCreate for CreateItem {}
impl PrepCreate for CreateTemplate {}
impl PrepCreate for CreateInvoice {}

// --- Validators ---
impl ValidSize for CreateCompany {}
impl ValidImage for CreateCompany {}

impl ValidSize for CreateMethod {}
impl ValidImage for CreateMethod {}

// --- TableNames ---
impl TableName for CreateCompany {
    fn table_name(&self) -> String {
        "company".to_string()
    }
}

impl TableName for CreateClient {
    fn table_name(&self) -> String {
        "client".to_string()
    }
}

impl TableName for CreateTerms {
    fn table_name(&self) -> String {
        "terms".to_string()
    }
}

impl TableName for CreateMethod {
    fn table_name(&self) -> String {
        "methods".to_string()
    }
}
impl TableName for CreateItem {
    fn table_name(&self) -> String {
        "items".to_string()
    }
}

impl TableName for CreateTemplate {
    fn table_name(&self) -> String {
        "templates".to_string()
    }
}

impl TableName for CreateInvoice {
    fn table_name(&self) -> String {
        "invoices".to_string()
    }
}

// +++ PrepFields +++
impl PrepFields for CreateCompany {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("name".to_string());
        if self.logo.is_some() {
            fnames.push("logo".to_string());
        }
        if let Some(contact) = &self.contact {
            fnames.extend(contact.fields());
        }
        fnames
    }
}

impl PrepFields for CreateClient {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("name".to_string());
        if let Some(contact) = &self.contact {
            fnames.extend(contact.fields());
        }
        fnames
    }
}

impl PrepFields for CreateTerms {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("name".to_string());
        fnames.push("due".to_string());
        fnames
    }
}

impl PrepFields for CreateMethod {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("name".to_string());
        if self.link.is_some() {
            fnames.push("link".to_string());
        }
        if self.qr.is_some() {
            fnames.push("qr".to_string());
        }
        fnames
    }
}

impl PrepFields for CreateItem {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("name".to_string());
        fnames.push("rate".to_string());
        fnames
    }
}

impl PrepFields for CreateTemplate {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("name".to_string());
        fnames.push("company_id".to_string());
        fnames.push("client_id".to_string());
        fnames.push("terms_id".to_string());
        fnames.push("methods_json".to_string());
        fnames
    }
}

impl PrepFields for CreateInvoice {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("template_id".to_string());
        fnames.push("date".to_string());
        fnames.push("show_methods".to_string());
        fnames.push("show_notes".to_string());
        fnames.push("stage".to_string());
        fnames.push("status".to_string());
        fnames.push("status_date".to_string());
        fnames.push("status_check".to_string());
        fnames.push("notes".to_string());
        fnames.push("items_json".to_string());
        fnames
    }
}

// ~~ PrepValues ~~
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
