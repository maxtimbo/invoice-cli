use crate::cli::contact::Contact;
use crate::db::prepare::{PrepFields, PrepValues, TableName, PrepCreate};
use std::path::PathBuf;
use serde_json;

use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum CreateCommands {
    Company(CreateCompany),
    Client(CreateClient),
    Terms(CreateTerms),
    Method(CreateMethod),
    Item(CreateItem),
}

#[derive(Debug, Args)]
#[group(required = false)]
pub struct CreateCompany {
    pub name: String,

    #[arg(long)]
    pub logo: Option<PathBuf>,

    #[command(flatten)]
    pub contact: Contact,
}

#[derive(Debug, Args)]
#[group(required = false)]
pub struct CreateClient {
    pub name: String,

    #[command(flatten)]
    pub contact: Contact,
}

#[derive(Debug, Args)]
pub struct CreateTerms {
    pub name: String,
    #[arg(long)]
    pub due: u32,
}

#[derive(Debug, Args)]
pub struct CreateMethod {
    pub name: String,
}

#[derive(Debug, Args)]
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

impl PrepCreate for CreateCompany {}
impl PrepCreate for CreateClient {}
impl PrepCreate for CreateTerms {}
impl PrepCreate for CreateMethod {}
impl PrepCreate for CreateItem {}
impl PrepCreate for CreateTemplate {}

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
        "item".to_string()
    }
}

impl TableName for CreateTemplate {
    fn table_name(&self) -> String {
        "templates".to_string()
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

// ~~ PrepValues ~~
impl PrepValues for CreateCompany {
    fn values(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<rusqlite::types::Value> = Vec::new();
        values.push(self.name.clone().into());
        if let Some(logo) = &self.logo {
            values.push(rusqlite::types::Value::Blob(std::fs::read(logo).unwrap()));
            //values.push(self.logo.clone().into());
        }
        values.extend(self.contact.values());
        values
    }
}

impl PrepValues for CreateClient {
    fn values(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<rusqlite::types::Value> = Vec::new();
        values.push(self.name.clone().into());
        values.extend(self.contact.values());
        values
    }
}

impl PrepValues for CreateTerms {
    fn values(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<rusqlite::types::Value> = Vec::new();
        values.push(self.name.clone().into());
        values.push(self.due.into());
        values
    }
}

impl PrepValues for CreateMethod {
    fn values(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<rusqlite::types::Value> = Vec::new();
        values.push(self.name.clone().into());
        values
    }
}

impl PrepValues for CreateItem {
    fn values(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<rusqlite::types::Value> = Vec::new();
        values.push(self.name.clone().into());
        values.push(self.rate.into());
        values
    }
}

impl PrepValues for CreateTemplate {
    fn values(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<rusqlite::types::Value> = Vec::new();
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


