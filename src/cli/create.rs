use std::path::PathBuf;

use chrono::NaiveDate;
use clap::{Args, Subcommand};
use rusqlite::types::Value;
use serde_json;
use serde::Deserialize;

use crate::cli::contact::Contact;
use crate::models::invoice::InvoiceItem;
use crate::db::prepare::{PrepFields, PrepValues, TableName, PrepCreate};
use crate::validators::{ValidSize, ValidImage};

#[derive(Subcommand)]
pub enum CreateCommands {
    FromJson(FromJSON),
    Company(CreateCompany),
    Client(CreateClient),
    Terms(CreateTerms),
    Method(CreateMethod),
    Item(CreateItem),
}

#[derive(Debug, Args)]
pub struct FromJSON {
    pub json_input: PathBuf,
}

#[derive(Debug, Args, Deserialize)]
#[group(required = false)]
pub struct CreateCompany {
    pub name: String,

    #[arg(long)]
    pub logo: Option<PathBuf>,

    #[command(flatten)]
    pub contact: Contact,
}

#[derive(Debug, Args, Deserialize)]
#[group(required = false)]
pub struct CreateClient {
    pub name: String,

    #[command(flatten)]
    pub contact: Contact,
}

#[derive(Debug, Args, Deserialize)]
pub struct CreateTerms {
    pub name: String,
    #[arg(long)]
    pub due: u32,
}

#[derive(Debug, Args, Deserialize)]
pub struct CreateMethod {
    pub name: String,
}

#[derive(Debug, Args, Deserialize)]
pub struct CreateItem {
    pub name: String,
    #[arg(long)]
    pub rate: i32,
}

#[derive(Debug)]
pub struct CreateTemplate {
    pub name: String,
    pub company: i64,
    pub client: i64,
    pub terms: i64,
    pub methods: Vec<i64>,
}

#[derive(Debug)]
pub struct CreateInvoice {
    pub template: i64,
    pub date: NaiveDate,
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
        fnames.extend(self.contact.fields());
        fnames
    }
}

impl PrepFields for CreateClient {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("name".to_string());
        fnames.extend(self.contact.fields());
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
        values.extend(self.contact.values());
        values
    }
}

impl PrepValues for CreateClient {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        values.push(self.name.clone().into());
        values.extend(self.contact.values());
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
        values
    }
}

impl PrepValues for CreateItem {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        values.push(self.name.clone().into());
        values.push(self.rate.into());
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
        let methods_json = serde_json::to_string(&self.methods).expect("Failed to serialize to JSON");
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
        let items_json = serde_json::to_string(&self.items).expect("Failed to serialize to JSON");
        values.push(items_json.into());
        values
    }
}
