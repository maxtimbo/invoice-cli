use clap::Subcommand;
use invoice_cli::select_entity;
use crate::db::InvoiceDB;

#[derive(Debug, Subcommand, PartialEq)]
#[group(required = true, multiple = false)]
pub enum ListFlags {
    Company,
    Client,
    Terms,
    Methods,
    Items,
    Templates,
    Invoices,
}

pub fn handle_list(flags: &ListFlags, db: &InvoiceDB) -> Result<(), anyhow::Error> {
    match flags {
        ListFlags::Company => {
            let id = select_entity!("Select Company:", db, "company")?;
            let entity = db.get_company(&id)?;
            println!("{}", entity);
        },
        ListFlags::Client => {
            let id = select_entity!("Select Client:", db, "client")?;
            let entity = db.get_client(&id)?;
            println!("{}", entity);
        },
        ListFlags::Methods => {
            let id = select_entity!("Select Payment Method:", db, "methods")?;
            let entity = db.get_method(&id)?;
            println!("{}", entity);
        },
        ListFlags::Terms => {
            let id = select_entity!("Select Terms:", db, "terms")?;
            let entity = db.get_terms(&id)?;
            println!("{}", entity);
        },
        ListFlags::Items => {
            let id = select_entity!("Select Item:", db, "items")?;
            let entity = db.get_item(&id)?;
            println!("{}", entity);
        },
        ListFlags::Templates => {
            let id = select_entity!("Select Template:", db, "templates")?;
            let entity = db.get_template(&id)?;
            println!("{}", entity);
        },
        ListFlags::Invoices => {
            let id = select_entity!("Select Invoices:", db, "invoices")?;
            let entity = db.get_invoice(&id)?;
            println!("{}", entity);
        }
    }
    Ok(())
}
