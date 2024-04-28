//use crate::cli::list::*;
use crate::cli::create::{CreateTemplate, CreateInvoice};
use invoice_cli::{select_entity, select_multiple_entities};
use crate::db::InvoiceDB;
use anyhow::Result;

use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum GenerateCommands {
    Template(GenerateTemplate),
    Invoice(GenerateInvoice),
}

#[derive(Debug, Args)]
pub struct GenerateTemplate {
    pub name: String,
}

impl GenerateTemplate {
    pub fn generate(&self, db: &InvoiceDB) -> Result<CreateTemplate> {
        let company_selection = select_entity!("Select Company:", db, "company")?;
        let client_selection = select_entity!("Select Client:", db, "client")?;
        let terms_selection = select_entity!("Select Payment Terms:", db, "terms")?;
        let methods_selection = select_multiple_entities!("Select Payment Methods:", db, "methods")?;
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

#[derive(Debug, Args)]
pub struct GenerateInvoice {
    //pub template: i64,
    
    #[arg(long, short)]
    pub output: Option<PathBuf>,
}

impl GenerateInvoice {
    pub fn generate(&self, db: &InvoiceDB) -> Result<CreateInvoice> {
        let template_selection = select_entity!("Select Template:", db, "template")?;
        //let items_selection = select_multiple_entities!("Add items to the invoice:", db, "items")?;
        //let quantities = todo!("Specify the quantity for item [x]");
        let new_invoice = CreateInvoice {
            template: template_selection,
        };

        Ok(new_invoice)
    }
}

