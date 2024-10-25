use std::path::PathBuf;
use std::str::FromStr;

use clap::{Args, Subcommand};
use inquire::{DateSelect, Confirm, Select};
use anyhow::Result;

use crate::cli::create::{CreateInvoice, CreateTemplate};
use crate::db::InvoiceDB;
use crate::db::prepare::PrepCreate;
use crate::models::editor_optional;
use crate::models::invoice::{InvoiceItem, InvoiceAttrs, InvoiceStage, PaidStatus};
use crate::render::TemplateEngine;
use crate::commands::selectors::EntitySelector;

#[derive(Debug, Subcommand, PartialEq)]
pub enum GenerateCommands {
    Template(GenerateTemplate),
    Invoice(GenerateInvoice),
}

pub fn handle_generate(gen: &GenerateCommands, db: &InvoiceDB, renderer: &TemplateEngine) -> Result<(), anyhow::Error> {
    match gen {
        GenerateCommands::Template(obj) => {
            let template = GenerateTemplate::generate(obj, &db)?;
            db.create_entry(template.prepare())?;
        }
        GenerateCommands::Invoice(obj) => {
            match (&obj.id, &obj.output) {
                (Some(id), Some(output)) => {
                    let invoice_obj = db.get_invoice(id)?;
                    let render = renderer.render(&invoice_obj)?;
                    renderer.to_file(&render, output)?;
                    let pdf = renderer.to_pdf(&output)?;
                    open::that(pdf)?;
                }
                (None, Some(output)) => {
                    let invoice = GenerateInvoice::generate(obj, &db)?;
                    let new_invoice = db.create_entry(invoice.prepare())?;
                    let invoice_obj = db.get_invoice(&new_invoice)?;
                    let render = renderer.render(&invoice_obj)?;
                    renderer.to_file(&render, output)?;
                    let pdf = renderer.to_pdf(&output)?;
                    open::that(pdf)?;
                }
                (Some(id), None) => {
                    let invoice_obj = db.get_invoice(id)?;
                    let output = std::path::PathBuf::from(
                        format!("Invoice{}_{}.html",
                                invoice_obj.id.to_string(),
                                invoice_obj.date.to_string()
                                )
                        );
                    let render = renderer.render(&invoice_obj)?;
                    renderer.to_file(&render, &output)?;
                    let pdf = renderer.to_pdf(&output)?;
                    open::that(pdf)?;
                }
                (None, None) => {
                    let invoice = GenerateInvoice::generate(obj, &db)?;
                    let id = db.create_entry(invoice.prepare())?;
                    let invoice_obj = db.get_invoice(&id)?;
                    let output = std::path::PathBuf::from(
                        format!("Invoice{}_{}.html",
                                invoice_obj.id.to_string(),
                                invoice_obj.date.to_string()
                                )
                        );
                    let render = renderer.render(&invoice_obj)?;
                    renderer.to_file(&render, &output)?;
                    let pdf = renderer.to_pdf(&output)?;
                    open::that(pdf)?;
                }
            };
        }
    }
    Ok(())
}

#[derive(Debug, Args, PartialEq)]
pub struct GenerateTemplate {
    pub name: String,
}

impl GenerateTemplate {
    pub fn generate(&self, db: &InvoiceDB) -> Result<CreateTemplate> {
        //let company_selection = select_entity!("Select Company:", db, "company")?;
        let company_selection = EntitySelector::new(db, "company", "Select Company:", true).select_entity()?;
        let client_selection = EntitySelector::new(db, "client", "Select Client:", true).select_entity()?;
        let terms_selection = EntitySelector::new(db, "terms", "Select Payment Terms:", true).select_entity()?;
        let methods_selection = EntitySelector::new(db, "methods", "Select Payment Methods:", true).multi_select_entity()?;
        let new_template = CreateTemplate {
            name: self.name.clone(),
            company: company_selection,
            client: client_selection,
            terms: terms_selection,
            methods: methods_selection,
        };
        Ok(new_template)
    }
}

#[derive(Debug, Args, PartialEq)]
pub struct GenerateInvoice {
    pub id: Option<i64>,
    #[arg(long, short)]
    pub output: Option<PathBuf>,
}

impl GenerateInvoice {
    pub fn generate(&self, db: &InvoiceDB) -> Result<CreateInvoice> {
        let date = DateSelect::new("Invoice date").prompt()?;
        let template = EntitySelector::new(db, "templates", "Select Template:", false).select_entity()?;
        let show_methods = Confirm::new("Show payment method?")
            .with_default(false)
            .prompt()?;
        let show_notes = Confirm::new("Show notes?")
            .with_default(true)
            .prompt()?;
        let stages = vec!["Quote", "Invoice"];
        let selected_stage = Select::new("Select invoice stage:", stages).prompt()?;
        
        let stage = InvoiceStage::from_str(&selected_stage)
            .map_err(|err| anyhow::anyhow!(err))?;

        let statuses = vec!["Waiting", "Past Due", "Paid", "Failed", "Refunded"];
        let selected_status = Select::new("Select payment status:", statuses).prompt()?;

        let status = PaidStatus::from_str(&selected_status)
            .map_err(|err| anyhow::anyhow!(err))?;

        let attributes = InvoiceAttrs {
            show_methods,
            show_notes,
            stage,
            status,
        };

        let notes = editor_optional("Enter notes about the invoice, or enter None to leave it blank:", "")?;
        let item_ids = EntitySelector::new(db, "items", "Add items to the invoice:", true).multi_select_entity()?;
        let mut items = Vec::new();
        for item_id in item_ids {
            let item_short = &db.get_item(&item_id)?;
            let quantity: i64 = inquire::CustomType::<i64>::new(&format!(
                "Enter quantity for item {} - {}:",
                item_short.id, item_short.name
            ))
            .with_error_message("Please enter a valid integer")
            .prompt()?;
            items.push(InvoiceItem {
                item: item_id,
                quantity: quantity,
            });
        }
        let new_invoice = CreateInvoice {
            template,
            attributes,
            notes,
            date,
            items,
        };

        Ok(new_invoice)
    }
}
