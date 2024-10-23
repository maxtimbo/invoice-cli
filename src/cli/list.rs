use clap::Subcommand;
use crate::db::InvoiceDB;
use crate::commands::selectors::EntitySelector;

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
            let id = EntitySelector::new(db, "company", "Select Company", false).select_entity()?;
            let entity = db.get_company(&id)?;
            println!("{}", entity);
        },
        ListFlags::Client => {
            let id = EntitySelector::new(db, "client", "Select Client", false).select_entity()?;
            let entity = db.get_client(&id)?;
            println!("{}", entity);
        },
        ListFlags::Methods => {
            let id = EntitySelector::new(db, "methods", "Select Payment Method", false).select_entity()?;
            let entity = db.get_method(&id)?;
            println!("{}", entity);
        },
        ListFlags::Terms => {
            let id = EntitySelector::new(db, "terms", "Select Terms", false).select_entity()?;
            let entity = db.get_terms(&id)?;
            println!("{}", entity);
        },
        ListFlags::Items => {
            let id = EntitySelector::new(db, "items", "Select Item", false).select_entity()?;
            let entity = db.get_item(&id)?;
            println!("{}", entity);
        },
        ListFlags::Templates => {
            let id = EntitySelector::new(db, "templates", "Select Template", false).select_entity()?;
            let entity = db.get_template(&id)?;
            println!("{}", entity);
        },
        ListFlags::Invoices => {
            let id = EntitySelector::new(db, "invoices", "Select Invoices", false).select_entity()?;
            let entity = db.get_invoice(&id)?;
            println!("{}", entity);
        }
    }
    Ok(())
}
