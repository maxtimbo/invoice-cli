use crate::db::prepare::{PrepCreate, PrepUpdate, PrepDelete};
use invoice_cli::print_entries;
use crate::db::InvoiceDB;
mod contact;
mod create;
use crate::cli::create::*;
mod edit;
use crate::cli::edit::*;
mod list;
use crate::cli::list::*;
mod delete;
use crate::cli::delete::*;
mod generate;
use crate::cli::generate::*;


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
    Create(CreateCommands),

    #[command(subcommand)]
    List(ListFlags),

    #[command(subcommand)]
    Edit(EditCommands),

    #[command(subcommand)]
    Delete(DeleteCommands),

    #[command(subcommand)]
    Generate(GenerateCommands),
}

impl Cli {
    pub fn to_cmd(db: &mut InvoiceDB) -> Result<()> {
        let cli = Cli::parse();
        match &cli.commands {
            Commands::Create(create) => match create {
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
                            res.display();
                        },
                        None => {
                            let res = db.get_table("company")?;
                            println!("Companies:");
                            print_entries!(res);
                        }
                    }
                },
                ListFlags::Client(opt) => {
                    match &opt.id {
                        Some(value) => {
                            let res = db.get_client(value)?;
                            res.display();
                        },
                        None => {
                            let res = db.get_table("client")?;
                            println!("Clients:");
                            print_entries!(res);
                        }
                    }
                },
                ListFlags::Methods(opt) => {
                    match &opt.id {
                        Some(value) => {
                            let res = db.get_method(value)?;
                            res.display();
                        },
                        None => {
                            let res = db.get_table("methods")?;
                            println!("Methods:");
                            print_entries!(res);
                        }
                    }
                },
                ListFlags::Terms(opt) => {
                    match &opt.id {
                        Some(value) => {
                            let res = db.get_terms(value)?;
                            res.display();
                        },
                        None => {
                            let res = db.get_table("terms")?;
                            println!("Terms:");
                            print_entries!(res);
                        }
                    }
                },
                ListFlags::Items(opt) => {
                    match &opt.id {
                        Some(value) => {
                            let res = db.get_item(value)?;
                            res.display();
                        },
                        None => {
                            let res = db.get_table("items")?;
                            println!("Items:");
                            print_entries!(res);
                        }
                    }
                },
                ListFlags::Templates(opt) => {
                    match &opt.id {
                        Some(value) => {
                            let res = db.get_template(value)?;
                            res.display();
                        },
                        None => {
                            let res = db.get_table("templates")?;
                            println!("Templates:");
                            print_entries!(res);
                        }
                    }
                },
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
                }
            },
            Commands::Generate(gen) => match gen {
                GenerateCommands::Template(obj) => {
                    let template = GenerateTemplate::generate(obj, &db)?;
                    db.create_entry(template.prepare())?;
                },
                GenerateCommands::Invoice(obj) => {
                    let invoice = GenerateInvoice::generate(obj, &db)?;
                    db.create_entry(invoice.prepare())?;
                }
            }
        }
        Ok(())
    }
}
