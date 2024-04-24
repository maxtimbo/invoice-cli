use crate::cli::contact::Contact;
use crate::db::prepare::{PrepFields, PrepValues, TableName, PrepCreate};

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
    #[arg(long)]
    pub name: String,

    #[arg(long)]
    pub logo: Option<String>,
    //logo: Option<PathBuf>,

    #[command(flatten)]
    pub contact: Contact,
}

#[derive(Debug, Args)]
#[group(required = false)]
pub struct CreateClient {
    #[arg(long)]
    pub name: String,

    #[command(flatten)]
    pub contact: Contact,
}

#[derive(Debug, Args)]
pub struct CreateTerms {
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub due: u32,
}

#[derive(Debug, Args)]
pub struct CreateMethod {
    #[arg(long)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct CreateItem {
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub rate: i32,
}

impl PrepCreate for CreateCompany {}
impl PrepCreate for CreateClient {}
impl PrepCreate for CreateTerms {}
impl PrepCreate for CreateMethod {}
impl PrepCreate for CreateItem {}

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

// ~~ PrepValues ~~
impl PrepValues for CreateCompany {
    fn values(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<rusqlite::types::Value> = Vec::new();
        values.push(self.name.clone().into());
        if self.logo.is_some() {
            values.push(self.logo.clone().into());
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
