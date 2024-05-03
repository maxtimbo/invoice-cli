use crate::db::prepare::{PrepCreate, PrepUpdate, PrepDelete};
use crate::db::InvoiceDB;
mod contact;
pub mod create;
use crate::cli::create::*;
mod edit;
use crate::cli::edit::*;
mod list;
use crate::cli::list::*;
mod delete;
use crate::cli::delete::*;
mod generate;
use crate::cli::generate::*;
use crate::render::TemplateEngine;


use clap::{Parser, Subcommand};
use anyhow::Result;

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
                CreateCommands::FromJson(obj) => {
                    match FromJSON::from(obj) {
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
                        },
                        Err(e) => eprintln!("Failed to parse JSON: {}", e),
                    }
                }
                CreateCommands::Company(obj) => {
                    db.create_entry(CreateCompany::prepare(obj))?;
                },
                CreateCommands::Client(obj) => {
                    db.create_entry(CreateClient::prepare(obj))?;
                },
                CreateCommands::Terms(obj) => {
                    db.create_entry(CreateTerms::prepare(obj))?;
                },
                CreateCommands::Method(obj) => {
                    db.create_entry(CreateMethod::prepare(obj))?;
                },
                CreateCommands::Item(obj) => {
                    db.create_entry(CreateItem::prepare(obj))?;
                },
            },
            Commands::List(flags) => match flags {
                ListFlags::Company(opt) => {
                    match &opt.id {
                        Some(value) => {
                            let res = db.get_company(value)?;
                            println!("{}", res);
                        },
                        None => {
                            let res = db.get_table("company")?;
                            println!("Companies:");
                            for entry in &res { println!("{}", entry); }
                        }
                    }
                },
                ListFlags::Client(opt) => {
                    match &opt.id {
                        Some(value) => {
                            let res = db.get_client(value)?;
                            println!("{}", res);
                        },
                        None => {
                            let res = db.get_table("client")?;
                            println!("Clients:");
                            for entry in &res { println!("{}", entry); }
                        }
                    }
                },
                ListFlags::Methods(opt) => {
                    match &opt.id {
                        Some(value) => {
                            let res = db.get_method(value)?;
                            println!("{}", res);
                        },
                        None => {
                            let res = db.get_table("methods")?;
                            println!("Methods:");
                            for entry in &res { println!("{}", entry); }
                        }
                    }
                },
                ListFlags::Terms(opt) => {
                    match &opt.id {
                        Some(value) => {
                            let res = db.get_terms(value)?;
                            println!("{}", res);
                        },
                        None => {
                            let res = db.get_table("terms")?;
                            println!("Terms:");
                            for entry in &res { println!("{}", entry); }
                        }
                    }
                },
                ListFlags::Items(opt) => {
                    match &opt.id {
                        Some(value) => {
                            let res = db.get_item(value)?;
                            println!("{}", res);
                        },
                        None => {
                            let res = db.get_table("items")?;
                            println!("Items:");
                            for entry in &res { println!("{}", entry); }
                        }
                    }
                },
                ListFlags::Templates(opt) => {
                    match &opt.id {
                        Some(value) => {
                            let res = db.get_template(value)?;
                            println!("{}", res);
                        },
                        None => {
                            let res = db.get_table("templates")?;
                            println!("Templates:");
                            for entry in &res { println!("{}", entry); }
                        }
                    }
                },
                ListFlags::Invoices(opt) => {
                    match &opt.id {
                        Some(value) => {
                            let res = db.get_invoice(value)?;
                            println!("{}", res);
                        },
                        None => {
                            let res = db.get_table("invoices")?;
                            println!("Invoices:");
                            for entry in &res { println!("{}", entry); }
                        }
                    }
                }
            },
            Commands::Edit(edit) => match edit {
                EditCommands::Company(obj) => {
                    db.update_entry(EditCompany::prepare(obj), &obj.id)?;
                },
                EditCommands::Client(obj) => {
                    db.update_entry(EditClient::prepare(obj), &obj.id)?;
                },
                EditCommands::Terms(obj) => {
                    db.update_entry(EditTerms::prepare(obj), &obj.id)?;
                },
                EditCommands::Method(obj) => {
                    db.update_entry(EditMethod::prepare(obj), &obj.id)?;
                },
                EditCommands::Item(obj) => {
                    db.update_entry(EditItem::prepare(obj), &obj.id)?;
                },
            },
            Commands::Delete(arg) => match arg {
                DeleteCommands::Company(obj) => {
                    db.delete_entry(DeleteCompany::prepare(obj), &obj.id)?;
                },
                DeleteCommands::Client(obj) => {
                    db.delete_entry(DeleteClient::prepare(obj), &obj.id)?;
                },
                DeleteCommands::Terms(obj) => {
                    db.delete_entry(DeleteTerms::prepare(obj), &obj.id)?;
                },
                DeleteCommands::Method(obj) => {
                    db.delete_entry(DeleteMethod::prepare(obj), &obj.id)?;
                },
                DeleteCommands::Item(obj) => {
                    db.delete_entry(DeleteItem::prepare(obj), &obj.id)?;
                },
                DeleteCommands::Template(obj) => {
                    db.delete_entry(DeleteTemplate::prepare(obj), &obj.id)?;
                },
                DeleteCommands::Invoice(obj) => {
                    db.delete_entry(DeleteInvoice::prepare(obj), &obj.id)?;
                }
            },
            Commands::Generate(gen) => match gen {
                GenerateCommands::Template(obj) => {
                    let template = GenerateTemplate::generate(obj, &db)?;
                    db.create_entry(template.prepare())?;
                },
                GenerateCommands::Invoice(obj) => {
                    match (&obj.id, &obj.output) {
                        (Some(id),  Some(output))   => {
                            let invoice_obj = db.get_invoice(id)?;
                            let render = renderer.render(&invoice_obj)?;
                            renderer.to_file(&render, output)?;
                        },
                        (None,      Some(output))   => {
                            let invoice = GenerateInvoice::generate(obj, &db)?;
                            let new_invoice = db.create_entry(invoice.prepare())?;
                            let invoice_obj = db.get_invoice(&new_invoice)?;
                            let render = renderer.render(&invoice_obj)?;
                            renderer.to_file(&render, output)?;
                        },
                        (Some(id),  None)           => {
                            let invoice_obj = db.get_invoice(id)?;
                            let serialized = serde_json::to_string(&invoice_obj).expect("Failed to serialize.");
                            println!("Serialized Invoice: {}", serialized);
                        },
                        (None,      None)           => {
                            let invoice = GenerateInvoice::generate(obj, &db)?;
                            db.create_entry(invoice.prepare())?;
                        },
                    };
                }
            }
        }
        Ok(())
    }
}
