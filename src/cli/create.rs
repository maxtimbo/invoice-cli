use std::path::PathBuf;

use chrono::NaiveDate;
use clap::{Args, Subcommand};
use serde::Deserialize;
use rust_decimal::Decimal;

use crate::db::InvoiceDB;
use crate::db::prepare::{PrepUpdate, PrepCreate};
use crate::cli::contact::Contact;
use crate::models::{TableName, EntityUpdater};
use crate::models::invoice::{InvoiceItem, InvoiceAttrs};

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
    Terms { name: String },
    /// Create payment methods
    Method { name: String },
    /// Create inventory items
    Item { name: String },

}

pub fn handle_create(create: &CreateCommands, db: &InvoiceDB) -> Result<i64, anyhow::Error> {
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
                Ok(0)
            }
            Err(e) => Err(anyhow::anyhow!("Failed to parse JSON: {}", e)),
        },
        CreateCommands::Company{ name: obj } => {
            let create_company = CreateCompany::new(obj.to_string());
            //let create_company = CreateCompany { name: obj.to_string(), logo: None, contact: None };
            //let id = db.create_entry(create_company.prepare())?;
            //let entity = db.get_company(&id)?;
            //db.update_entry(entity.update()?.prepare(), &id)?;
            Ok(id)
        }
        CreateCommands::Client { name: obj } => {
            let create_client = CreateClient::new(obj.to_string());
            //let create_client = CreateClient { name: obj.to_string(), contact: None };
            //let id = db.create_entry(create_client.prepare())?;
            //let entity = db.get_client(&id)?;
            //db.update_entry(entity.update()?.prepare(), &id)?;
            Ok(id)
        }
        CreateCommands::Terms {name: obj } => {
            //let id = db.create_entry(CreateTerms::prepare(obj))?;
            let create_terms = CreateTerms::new(obj.to_string());
            Ok(id)
        }
        CreateCommands::Method { name: obj } => {
            let create_methods = CreateMethod::new(obj.to_string());
            // let create_method = CreateMethod { name: obj.to_string(), link: None, qr: None };
            // let id = db.create_entry(create_method.prepare())?;
            // let entity = db.get_method(&id)?;
            // db.update_entry(entity.update()?.prepare(), &id)?;
            Ok(id)
        }
        CreateCommands::Item { name: obj } => {
            let create_items = CreateItem::new(obj.to_string());
            //let id = db.create_entry(CreateItem::prepare(obj))?;
            Ok(id)
        }
    }
}

#[derive(Debug, Args, PartialEq)]
pub struct FromJSON {
    pub json_input: PathBuf,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CreateCompany {
    pub table_name: TableName,
    pub name: String,
}

impl CreateCompany {
    fn new(name: String) -> Self {
        Self {
            table_name: TableName::Company,
            name,
        }
    }
}


#[derive(Debug, Deserialize, PartialEq)]
pub struct CreateClient {
    pub table_name: TableName,
    pub name: String,
}

impl CreateClient {
    fn new(name: String) -> Self {
        Self {
            table_name: TableName::Client,
            name,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CreateTerms {
    pub table_name: TableName,
    pub name: String,
}

impl CreateTerms {
    fn new(name: String) -> Self {
        Self {
            table_name: TableName::Terms,
            name,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CreateMethod {
    pub table_name: TableName,
    pub name: String,
}

impl CreateMethod {
    fn new(name: String) -> Self {
        Self {
            table_name: TableName::Methods,
            name,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CreateItem {
    pub table_name: TableName,
    pub name: String,
}

impl CreateItem {
    fn new(name: String) -> Self {
        Self {
            table_name: TableName::Items,
            name,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CreateTemplate {
    pub table_name: TableName,
    pub name: String,
}

impl CreateTemplate {
    fn new(name: String) -> Self {
        Self {
            table_name: TableName::Templates,
            name,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CreateInvoice {
    pub table_name: TableName,
}

impl CreateInvoice {
    fn new() -> Self {
        Self {
            table_name: TableName::Invoices,
        }
    }
}
