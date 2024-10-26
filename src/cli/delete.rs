use clap::Subcommand;

use crate::db::prepare::{PrepDelete, TableName};
use crate::db::InvoiceDB;
use crate::models::EntityDeleter;
use crate::commands::selectors::EntitySelector;

#[derive(Debug, Subcommand, PartialEq)]
pub enum DeleteCommands {
    Company,
    Client,
    Terms,
    Method,
    Item,
    Template,
    Invoice,
}

pub fn handle_delete(arg: &DeleteCommands, db: &InvoiceDB) -> Result<(), anyhow::Error> {
    match arg {
        DeleteCommands::Company => {
            let id = EntitySelector::new(db, "company", "Select Company", false).select_entity()?;
            let entity = db.get_company(&id)?;
            println!("{}", entity);
            db.delete_entry(entity.delete()?.prepare(), &id)?;
        }
        DeleteCommands::Client => {
            let id = EntitySelector::new(db, "client", "Select Client", false).select_entity()?;
            let entity = db.get_client(&id)?;
            println!("{}", entity);
            db.delete_entry(entity.delete()?.prepare(), &id)?;
        }
        DeleteCommands::Terms => {
            let id = EntitySelector::new(db, "terms", "Select Terms", false).select_entity()?;
            let entity = db.get_terms(&id)?;
            println!("{}", entity);
            db.delete_entry(entity.delete()?.prepare(), &id)?;
        }
        DeleteCommands::Method => {
            let id = EntitySelector::new(db, "methods", "Select Payment Method", false).select_entity()?;
            let entity = db.get_method(&id)?;
            println!("{}", entity);
            db.delete_entry(entity.delete()?.prepare(), &id)?;
        }
        DeleteCommands::Item => {
            let id = EntitySelector::new(db, "items", "Select Item", false).select_entity()?;
            let entity = db.get_item(&id)?;
            println!("{}", entity);
            db.delete_entry(entity.delete()?.prepare(), &id)?;
        }
        DeleteCommands::Template => {
            let id = EntitySelector::new(db, "templates", "Select Template", false).select_entity()?;
            let entity = db.get_template(&id)?;
            println!("{}", entity);
            db.delete_entry(entity.delete()?.prepare(), &id)?;
        }
        DeleteCommands::Invoice => {
            let id = EntitySelector::new(db, "invoices", "Select Invoices", false).select_entity()?;
            let entity = db.get_invoice(&id)?;
            println!("{}", entity);
            db.delete_entry(entity.delete()?.prepare(), &id)?;
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct DeleteCompany {
    pub id: i64,
}

#[derive(Debug)]
pub struct DeleteClient {
    pub id: i64,
}

#[derive(Debug)]
pub struct DeleteTerms {
    pub id: i64,
}

#[derive(Debug)]
pub struct DeleteMethod {
    pub id: i64,
}

#[derive(Debug)]
pub struct DeleteItem {
    pub id: i64,
}
#[derive(Debug)]
pub struct DeleteTemplate {
    pub id: i64,
}
#[derive(Debug)]
pub struct DeleteInvoice {
    pub id: i64,
}

impl PrepDelete for DeleteCompany {}
impl PrepDelete for DeleteClient {}
impl PrepDelete for DeleteTerms {}
impl PrepDelete for DeleteMethod {}
impl PrepDelete for DeleteItem {}
impl PrepDelete for DeleteTemplate {}
impl PrepDelete for DeleteInvoice {}

// --- TableNames ---
impl TableName for DeleteCompany {
    fn table_name(&self) -> String {
        "company".to_string()
    }
}

impl TableName for DeleteClient {
    fn table_name(&self) -> String {
        "client".to_string()
    }
}

impl TableName for DeleteTerms {
    fn table_name(&self) -> String {
        "terms".to_string()
    }
}

impl TableName for DeleteMethod {
    fn table_name(&self) -> String {
        "methods".to_string()
    }
}
impl TableName for DeleteItem {
    fn table_name(&self) -> String {
        "items".to_string()
    }
}
impl TableName for DeleteTemplate {
    fn table_name(&self) -> String {
        "templates".to_string()
    }
}
impl TableName for DeleteInvoice {
    fn table_name(&self) -> String {
        "invoices".to_string()
    }
}
