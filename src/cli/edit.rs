use std::path::PathBuf;
use clap::{Args, Subcommand};
use rusqlite::types::Value;

use crate::cli::contact::Contact;
use crate::db::prepare::{TableName, PrepFields, PrepValues, PrepUpdate};
use crate::validators::{ValidSize, ValidImage};

#[derive(Debug, Subcommand)]
pub enum EditCommands {
    Company(EditCompany),
    Client(EditClient),
    Terms(EditTerms),
    Method(EditMethod),
    Item(EditItem),
}

#[derive(Debug, Args)]
#[group(required = false)]
pub struct EditCompany {
    pub id: i64,
    #[arg(long, short)]
    pub name: Option<String>,
    #[arg(long, short)]
    pub logo: Option<PathBuf>,
    #[command(flatten)]
    pub contact: Contact,
}

#[derive(Debug, Args)]
#[group(required = false)]
pub struct EditClient {
    pub id: i64,
    #[arg(long, short)]
    pub name: Option<String>,
    #[command(flatten)]
    pub contact: Contact,
}

#[derive(Debug, Args)]
pub struct EditTerms {
    pub id: i64,
    #[arg(long, short)]
    pub name: Option<String>,
    #[arg(long, short)]
    pub due: Option<u32>,
}

#[derive(Debug, Args)]
pub struct EditMethod {
    pub id: i64,
    #[arg(long, short)]
    pub name: Option<String>,
}

#[derive(Debug, Args)]
pub struct EditItem {
    pub id: i64,
    #[arg(long, short)]
    pub name: Option<String>,
    #[arg(long, short)]
    pub rate: Option<i32>,
}

impl PrepUpdate for EditCompany {}
impl PrepUpdate for EditClient {}
impl PrepUpdate for EditTerms {}
impl PrepUpdate for EditMethod {}
impl PrepUpdate for EditItem {}

// --- Validators ---
impl ValidSize for EditCompany {}
impl ValidImage for EditCompany {}

// --- TableNames ---
impl TableName for EditCompany {
    fn table_name(&self) -> String {
        "company".to_string()
    }
}

impl TableName for EditClient {
    fn table_name(&self) -> String {
        "client".to_string()
    }
}

impl TableName for EditTerms {
    fn table_name(&self) -> String {
        "name".to_string()
    }
}

impl TableName for EditMethod {
    fn table_name(&self) -> String {
        "methods".to_string()
    }
}

impl TableName for EditItem {
    fn table_name(&self) -> String {
        "items".to_string()
    }
}

// +++ PrepFields +++
impl PrepFields for EditCompany {
    fn fields(&self) -> Vec<std::string::String> {
       let mut fnames = Vec::new();
       fnames.push(self.id.to_string());
       if self.name.is_some() {
           fnames.push("name".to_string());
       }
       if self.logo.is_some() {
           fnames.push("logo".to_string());
       }
       fnames.extend(self.contact.fields());
       fnames
    }
}

impl PrepFields for EditClient {
    fn fields(&self) -> Vec<std::string::String> {
       let mut fnames = Vec::new();
       fnames.push(self.id.to_string());
       if self.name.is_some() {
           fnames.push("name".to_string());
       }
       fnames.extend(self.contact.fields());
       fnames
    }
}

impl PrepFields for EditTerms {
    fn fields(&self) -> Vec<std::string::String> {
       let mut fnames = Vec::new();
       fnames.push(self.id.to_string());
       if self.name.is_some() {
           fnames.push("name".to_string());
       }
       if self.due.is_some() {
           fnames.push("due".to_string());
       }
       fnames
    }
}

impl PrepFields for EditMethod {
    fn fields(&self) -> Vec<std::string::String> {
       let mut fnames = Vec::new();
       fnames.push(self.id.to_string());
       if self.name.is_some() {
           fnames.push("name".to_string());
       }
       fnames
    }
}

impl PrepFields for EditItem {
    fn fields(&self) -> Vec<std::string::String> {
       let mut fnames = Vec::new();
       fnames.push(self.id.to_string());
       if self.name.is_some() {
           fnames.push("name".to_string());
       }
       if self.rate.is_some() {
           fnames.push("rate".to_string());
       }
       fnames
    }
}

// ~~~ PrepValues ~~~
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
       values
    }
}

impl PrepValues for EditItem {
    fn values(&self) -> Vec<Value> {
       let mut values: Vec<Value> = Vec::new();
       if self.name.is_some() {
           values.push(self.name.clone().into());
       }
       if self.rate.is_some() {
           values.push(self.rate.into());
       }
       values
    }
}

