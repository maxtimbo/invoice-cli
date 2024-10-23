use clap::Subcommand;
use std::path::PathBuf;
use rust_decimal::Decimal;

use crate::db::InvoiceDB;
use crate::db::prepare::PrepUpdate;
use crate::models::EntityUpdater;
use crate::models::invoice::{InvoiceStage, PaidStatus};
use crate::cli::contact::Contact;
use crate::commands::selectors::EntitySelector;

#[derive(Debug, Subcommand, PartialEq)]
pub enum EditCommands {
    Company,
    Client,
    Terms,
    Method,
    Item,
    Template,
    Invoice,
}

pub fn handle_edit(edit: &EditCommands, db: &InvoiceDB) -> Result<(), anyhow::Error> {
    match edit {
        EditCommands::Company => {
            let id = EntitySelector::new(db, "company", "Select Company", false).select_entity()?;
            let entity = db.get_company(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        }
        EditCommands::Client => {
            let id = EntitySelector::new(db, "client", "Select Client", false).select_entity()?;
            let entity = db.get_client(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        }
        EditCommands::Terms => {
            let id = EntitySelector::new(db, "terms", "Select Terms", false).select_entity()?;
            let entity = db.get_terms(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        },
        EditCommands::Method => {
            let id = EntitySelector::new(db, "methods", "Select Payment Method", false).select_entity()?;
            let entity = db.get_method(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        }
        EditCommands::Item => {
            let id = EntitySelector::new(db, "items", "Select Item", false).select_entity()?;
            let entity = db.get_item(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        }
        EditCommands::Template => {
            let id = EntitySelector::new(db, "templates", "Select template", false).select_entity()?;
            let entity = db.get_template(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        }
        EditCommands::Invoice => {
            let id = EntitySelector::new(db, "invoices", "Select invoice", false).select_entity()?;
            let entity = db.get_invoice(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        }
    }
    Ok(())
}


#[derive(Debug)]
pub struct EditCompany {
    pub id: i64,
    pub name: Option<String>,
    pub logo: Option<PathBuf>,
    pub contact: Contact,
}

#[derive(Debug)]
pub struct EditClient {
    pub id: i64,
    pub name: Option<String>,
    pub contact: Contact,
}

#[derive(Debug)]
pub struct EditTerms {
    pub id: i64,
    pub name: Option<String>,
    pub due: Option<i64>,
}

#[derive(Debug)]
pub struct EditMethod {
    pub id: i64,
    pub name: Option<String>,
    pub link: Option<String>,
    pub qr: Option<PathBuf>,
}

#[derive(Debug)]
pub struct EditItem {
    pub id: i64,
    pub name: Option<String>,
    pub rate: Option<Decimal>,
}

#[derive(Debug)]
pub struct EditTemplate {
    pub id: i64,
    pub name: Option<String>,
    pub company: Option<i64>,
    pub client: Option<i64>,
    pub terms: Option<i64>,
    pub methods: Option<Vec<i64>>,
}

#[derive(Debug)]
pub struct EditInvoice {
    pub id: i64,
    pub show_methods: Option<bool>,
    pub show_notes: Option<bool>,
    pub stage: Option<InvoiceStage>,
    pub status: Option<PaidStatus>,
    pub notes: Option<String>,
}
