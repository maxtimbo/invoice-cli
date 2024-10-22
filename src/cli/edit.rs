use clap::Subcommand;
use rusqlite::types::Value;
use std::path::PathBuf;

use crate::db::InvoiceDB;
use invoice_cli::select_entity;
use crate::models::EntityUpdater;
use crate::models::invoice::{InvoiceStage, PaidStatus};
use crate::cli::contact::Contact;
use crate::db::prepare::{PrepFields, PrepUpdate, PrepValues, TableName};
use crate::validators::{ValidImage, ValidSize};
use invoice_cli::decimal_to_i64;
use rust_decimal::Decimal;

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
            let id = select_entity!("Select Company:", db, "company")?;
            let entity = db.get_company(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        }
        EditCommands::Client => {
            let id = select_entity!("Select Client:", db, "client")?;
            let entity = db.get_client(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        }
        EditCommands::Terms => {
            let id = select_entity!("Select Terms:", db, "terms")?;
            let entity = db.get_terms(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        },
        EditCommands::Method => {
            let id = select_entity!("Select Payment Method:", db, "methods")?;
            let entity = db.get_method(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        }
        EditCommands::Item => {
            let id = select_entity!("Select Item:", db, "items")?;
            let entity = db.get_item(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        }
        EditCommands::Template => {
            let id = select_entity!("Select template:", db, "templates")?;
            let entity = db.get_template(&id)?;
            db.update_entry(entity.update()?.prepare(), &id)?;
        }
        EditCommands::Invoice => {
            let id = select_entity!("Select invoice:", db, "invoices")?;
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

impl PrepUpdate for EditCompany {}
impl PrepUpdate for EditClient {}
impl PrepUpdate for EditTerms {}
impl PrepUpdate for EditMethod {}
impl PrepUpdate for EditItem {}
impl PrepUpdate for EditTemplate {}
impl PrepUpdate for EditInvoice {}

// --- Validators ---
impl ValidSize for EditCompany {}
impl ValidImage for EditCompany {}
impl ValidSize for EditMethod {}
impl ValidImage for EditMethod {}

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
        "terms".to_string()
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

impl TableName for EditTemplate {
    fn table_name(&self) -> String {
        "templates".to_string()
    }
}

impl TableName for EditInvoice {
    fn table_name(&self) -> String {
        "invoices".to_string()
    }
}

// +++ PrepFields +++
impl PrepFields for EditCompany {
    fn fields(&self) -> Vec<String> {
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
    fn fields(&self) -> Vec<String> {
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
    fn fields(&self) -> Vec<String> {
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
    fn fields(&self) -> Vec<String> {
        let mut fnames = Vec::new();
        fnames.push(self.id.to_string());
        if self.name.is_some() {
            fnames.push("name".to_string());
        }
        if self.link.is_some() {
            fnames.push("link".to_string());
        }
        if self.qr.is_some() {
            fnames.push("qr".to_string());
        }
        fnames
    }
}

impl PrepFields for EditItem {
    fn fields(&self) -> Vec<String> {
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

impl PrepFields for EditTemplate {
    fn fields(&self) -> Vec<String> {
        let mut fnames = Vec::new();
        fnames.push(self.id.to_string());
        if self.name.is_some() {
            fnames.push("name".to_string());
        }
        if self.company.is_some() {
            fnames.push("company_id".to_string());
        }
        if self.client.is_some() {
            fnames.push("client_id".to_string());
        }
        if self.terms.is_some() {
            fnames.push("terms_id".to_string());
        }
        if self.methods.is_some() {
            fnames.push("methods_json".to_string());
        }
        fnames
    }
}

impl PrepFields for EditInvoice {
    fn fields(&self) -> Vec<String> {
        let mut fnames = Vec::new();
        fnames.push(self.id.to_string());
        if self.show_methods.is_some() {
            fnames.push("show_methods".to_string());
        }
        if self.show_notes.is_some() {
            fnames.push("show_notes".to_string());
        }
        if self.stage.is_some() {
            fnames.push("stage".to_string());
        }

        if self.status.is_some() {
            fnames.push("status".to_string());
            if let Some(PaidStatus::Paid { .. }) = &self.status {
                fnames.push("status_date".to_string());
                fnames.push("status_check".to_string());
            }
            if let Some(PaidStatus::Failed { .. }) | Some(PaidStatus::Refunded { .. }) = &self.status{
                fnames.push("status_date".to_string());
            }
        }

        if self.notes.is_some() {
            fnames.push("notes".to_string());
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
        if self.link.is_some() {
            values.push(self.link.clone().into());
        }
        if let Some(qr) = &self.qr {
            if self.is_valid_image(&qr) {
                match self.read_image(&qr) {
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

impl PrepValues for EditItem {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        if self.name.is_some() {
            values.push(self.name.clone().into());
        }
        if let Some(rate) = self.rate {
            values.push(decimal_to_i64!(rate).into());
        }
        values
    }
}

impl PrepValues for EditTemplate {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        if self.name.is_some() {
            values.push(self.name.clone().into());
        }
        if self.company.is_some() {
            values.push(self.company.into());
        }
        values
    }
}

impl PrepValues for EditInvoice {
    fn values(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();
        if self.show_methods.is_some() {
            values.push(self.show_methods.into());
        }
        if self.show_notes.is_some() {
            values.push(self.show_notes.into());
        }
        if let Some(stage) = &self.stage {
            match stage {
                InvoiceStage::Quote => { values.push("Quote".to_string().into()) }
                InvoiceStage::Invoice => { values.push("Invoice".to_string().into()) }
            }
        }
        if let Some(status) = &self.status {
            match status {
                PaidStatus::Waiting => { values.push("Waiting".to_string().into()) }
                PaidStatus::Paid { date, check } => {
                    values.push("Paid".to_string().into());
                    values.push(date.to_string().into());
                    if let Some(check_str) = check {
                        values.push(check_str.to_string().into());
                    } else {
                        values.push("None".to_string().into());
                    }
                }
                PaidStatus::Failed { date } => {
                    values.push("Failed".to_string().into());
                    values.push(date.to_string().into());
                }
                PaidStatus::Refunded { date } => {
                    values.push("Refunded".to_string().into());
                    values.push(date.to_string().into());
                }
            }
        }
        if let Some(notes) = &self.notes {
            values.push(notes.to_string().into());
        }
            
        values
    }
}
