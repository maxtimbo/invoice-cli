use crate::db::prepare::{PrepCreate, PrepDelete, PrepUpdate};
use crate::db::InvoiceDB;
pub mod contact;
pub mod create;
use crate::cli::create::*;
pub mod edit;
use crate::cli::edit::*;
mod list;
use crate::cli::list::*;
pub mod delete;
use crate::cli::delete::*;
mod generate;
use crate::cli::generate::*;
use crate::render::TemplateEngine;
use invoice_cli::select_entity;
use crate::models::{EntityDeleter, EntityUpdater};

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    /// Create an entity
    Create(CreateCommands),

    #[command(subcommand)]
    /// List entities
    List(ListFlags),

    #[command(subcommand)]
    /// Edit or Update entities
    Edit(EditCommands),

    #[command(subcommand)]
    /// Delete entities
    Delete(DeleteCommands),

    #[command(subcommand)]
    /// Generate a template or an invoice
    Generate(GenerateCommands),
}

impl Cli {
    pub fn to_cmd(db: &mut InvoiceDB, renderer: &TemplateEngine) -> Result<()> {
        let cli = Cli::parse();
        match &cli.commands {
            Commands::Create(create) => match create {
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
                    }
                    Err(e) => eprintln!("Failed to parse JSON: {}", e),
                },
                CreateCommands::Company(obj) => {
                    db.create_entry(CreateCompany::prepare(obj))?;
                }
                CreateCommands::Client(obj) => {
                    db.create_entry(CreateClient::prepare(obj))?;
                }
                CreateCommands::Terms(obj) => {
                    db.create_entry(CreateTerms::prepare(obj))?;
                }
                CreateCommands::Method(obj) => {
                    db.create_entry(CreateMethod::prepare(obj))?;
                }
                CreateCommands::Item(obj) => {
                    db.create_entry(CreateItem::prepare(obj))?;
                }
            },
            Commands::List(flags) => match flags {
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
                },
            },
            Commands::Edit(edit) => match edit {
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
            },
            Commands::Delete(arg) => match arg {
                DeleteCommands::Company => {
                    let id = select_entity!("Select Company:", db, "company")?;
                    let entity = db.get_company(&id)?;
                    println!("{}", entity);
                    db.delete_entry(entity.delete()?.prepare(), &id)?;
                }
                DeleteCommands::Client => {
                    let id = select_entity!("Select Client:", db, "client")?;
                    let entity = db.get_client(&id)?;
                    println!("{}", entity);
                    db.delete_entry(entity.delete()?.prepare(), &id)?;
                }
                DeleteCommands::Terms => {
                    let id = select_entity!("Select Terms:", db, "terms")?;
                    let entity = db.get_terms(&id)?;
                    println!("{}", entity);
                    db.delete_entry(entity.delete()?.prepare(), &id)?;
                }
                DeleteCommands::Method => {
                    let id = select_entity!("Select Payment Method:", db, "methods")?;
                    let entity = db.get_method(&id)?;
                    println!("{}", entity);
                    db.delete_entry(entity.delete()?.prepare(), &id)?;
                }
                DeleteCommands::Item => {
                    let id = select_entity!("Select Item:", db, "items")?;
                    let entity = db.get_item(&id)?;
                    println!("{}", entity);
                    db.delete_entry(entity.delete()?.prepare(), &id)?;
                }
                DeleteCommands::Template => {
                    let id = select_entity!("Select Template:", db, "templates")?;
                    let entity = db.get_template(&id)?;
                    println!("{}", entity);
                    db.delete_entry(entity.delete()?.prepare(), &id)?;
                }
                DeleteCommands::Invoice => {
                    let id = select_entity!("Select Invoices:", db, "invoices")?;
                    let entity = db.get_invoice(&id)?;
                    println!("{}", entity);
                    db.delete_entry(entity.delete()?.prepare(), &id)?;
                }
            },
            Commands::Generate(gen) => match gen {
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
                        }
                        (None, Some(output)) => {
                            let invoice = GenerateInvoice::generate(obj, &db)?;
                            let new_invoice = db.create_entry(invoice.prepare())?;
                            let invoice_obj = db.get_invoice(&new_invoice)?;
                            let render = renderer.render(&invoice_obj)?;
                            renderer.to_file(&render, output)?;
                        }
                        (Some(id), None) => {
                            let invoice_obj = db.get_invoice(id)?;
                            let serialized =
                                serde_json::to_string(&invoice_obj).expect("Failed to serialize.");
                            println!("Serialized Invoice: {}", serialized);
                        }
                        (None, None) => {
                            let invoice = GenerateInvoice::generate(obj, &db)?;
                            db.create_entry(invoice.prepare())?;
                        }
                    };
                }
            },
        }
        Ok(())
    }
}
