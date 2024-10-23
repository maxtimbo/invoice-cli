use std::path::PathBuf;

use chrono::NaiveDate;
use clap::{Args, Subcommand};
use serde::Deserialize;
use rust_decimal::Decimal;

use crate::db::InvoiceDB;
use crate::cli::contact::Contact;
use crate::models::EntityUpdater;
use crate::models::invoice::{InvoiceItem, InvoiceAttrs};
use crate::db::prepare::{PrepUpdate, PrepCreate};

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
            let create_company = CreateCompany { name: obj.to_string(), logo: None, contact: None };
            let id = db.create_entry(create_company.prepare())?;
            let entity = db.get_company(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
            Ok(id)
        }
        CreateCommands::Client { name: obj } => {
            let create_client = CreateClient { name: obj.to_string(), contact: None };
            let id = db.create_entry(create_client.prepare())?;
            let entity = db.get_client(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
            Ok(id)
        }
        CreateCommands::Terms(obj) => {
            let id = db.create_entry(CreateTerms::prepare(obj))?;
            Ok(id)
        }
        CreateCommands::Method { name: obj } => {
            let create_method = CreateMethod { name: obj.to_string(), link: None, qr: None };
            let id = db.create_entry(create_method.prepare())?;
            let entity = db.get_method(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
            Ok(id)
        }
        CreateCommands::Item(obj) => {
            let id = db.create_entry(CreateItem::prepare(obj))?;
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
