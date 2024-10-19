//use crate::cli::list::*;
use crate::cli::create::{CreateInvoice, CreateTemplate};
use crate::db::InvoiceDB;
use crate::models::prompt_optional;
use crate::models::invoice::{InvoiceItem, InvoiceAttrs, InvoiceStage, PaidStatus};
use anyhow::Result;
use invoice_cli::{select_entity, select_multiple_entities};
use crate::render::TemplateEngine;
use crate::db::prepare::PrepCreate;

use clap::{Args, Subcommand};
use inquire::{DateSelect, Confirm, Select};
use std::path::PathBuf;

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
                    open::that(output)?;
                }
                (None, Some(output)) => {
                    let invoice = GenerateInvoice::generate(obj, &db)?;
                    let new_invoice = db.create_entry(invoice.prepare())?;
                    let invoice_obj = db.get_invoice(&new_invoice)?;
                    let render = renderer.render(&invoice_obj)?;
                    renderer.to_file(&render, output)?;
                    open::that(output)?;
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
                    open::that(output)?;
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
                    open::that(output)?;

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
        let company_selection = select_entity!("Select Company:", db, "company")?;
        let client_selection = select_entity!("Select Client:", db, "client")?;
        let terms_selection = select_entity!("Select Payment Terms:", db, "terms")?;
        let methods_selection =
            select_multiple_entities!("Select Payment Methods:", db, "methods")?;
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
        let template = select_entity!("Select Template:", db, "templates")?;
        let show_methods = Confirm::new("Show payment method?")
            .with_default(false)
            .prompt()?;
        let show_notes = Confirm::new("Show notes?")
            .with_default(true)
            .prompt()?;
        let stages = vec!["Quote", "Invoice"];
        let selected_stage = Select::new("Select invoice stage:", stages).prompt()?;
        
        let stage = match selected_stage {
            "Quote" => InvoiceStage::Quote,
            "Invoice" => InvoiceStage::Invoice,
            _ => {
                println!("Error: Invalid stage");
                return Err(anyhow::anyhow!("Invalid stage selected"));
            }
        };

        let statuses = vec!["Waiting", "Paid", "Failed", "Refunded"];
        let selected_status = Select::new("Select payment status:", statuses).prompt()?;

        let status = match selected_status {
            "Waiting" => PaidStatus::Waiting,
            "Paid" => {
                let date = DateSelect::new("Select payment date").prompt()?;
                let check = prompt_optional("Enter check number if acclicable or enter 'None':", "")?;
                PaidStatus::Paid{ date: date.format("%Y%m%d").to_string(), check }
            },
            "Failed" => {
                let date = DateSelect::new("Select failed payment date").prompt()?;
                PaidStatus::Failed{ date: date.format("%Y%m%d").to_string() }
            },
            "Refunded" => {
                let date = DateSelect::new("Select payment refunded date:").prompt()?;
                PaidStatus::Refunded{ date: date.format("%Y%m%d").to_string() }
            },
            _ => {
                println!("Error: Invalid payment status");
                return Err(anyhow::anyhow!("Invalid status selected"));
            }
        };


        let attributes = InvoiceAttrs {
            show_methods,
            show_notes,
            stage,
            status,
        };
        let notes = prompt_optional("Enter notes about the invoice, or enter None to leave it blank:", "")?;
        let item_ids = select_multiple_entities!("Add items to the invoice:", db, "items")?;
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
