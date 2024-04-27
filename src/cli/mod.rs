use crate::db::prepare::{PrepCreate, PrepUpdate, PrepDelete};
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
    pub fn to_cmd(db: &InvoiceDB) -> Result<()> {
        let cli = Cli::parse();
        match &cli.commands {
            Commands::Create(create) => match create {
                CreateCommands::Company(company) => {
                    let mut query = CreateCompany::prepare(company);
                    query.create_entry(&db)?;
                },
                CreateCommands::Client(client) => {
                    let mut query = CreateClient::prepare(client);
                    query.create_entry(&db)?;
                },
                CreateCommands::Terms(terms) => {
                    let mut query = CreateTerms::prepare(terms);
                    query.create_entry(&db)?;
                },
                CreateCommands::Method(method) => {
                    let mut query = CreateMethod::prepare(method);
                    query.create_entry(&db)?;
                },
                CreateCommands::Item(item) => {
                    let mut query = CreateItem::prepare(item);
                    query.create_entry(&db)?;
                },
            },
            Commands::List(flags) => match flags {
                ListFlags::Company(opt) => {
                    println!("{:?}", flags);
                    println!("{:?}", opt);
                    let query = ListCompany::table_or_id(opt, opt.id);
                    query.list(&db, opt.id)?;
                },
                ListFlags::Client(opt) => {
                    let query = ListClient::table_or_id(opt, opt.id);
                    query.list(&db, opt.id)?;
                },
                ListFlags::Methods(opt) => {
                    let query = ListMethods::table_or_id(opt, opt.id);
                    query.list(&db, opt.id)?;
                },
                ListFlags::Terms(opt) => {
                    let query = ListTerms::table_or_id(opt, opt.id);
                    query.list(&db, opt.id)?;
                },
                ListFlags::Items(opt) => {
                    let query = ListItems::table_or_id(opt, opt.id);
                    query.list(&db, opt.id)?;
                },
            },
            Commands::Edit(edit) => match edit {
                EditCommands::Company(obj) => {
                    let mut query = EditCompany::prepare(obj);
                    query.update_entry(&db, &obj.id)?;
                },
                EditCommands::Client(obj) => {
                    let mut query = EditClient::prepare(obj);
                    query.update_entry(&db, &obj.id)?;
                },
                EditCommands::Terms(obj) => {
                    let mut query = EditTerms::prepare(obj);
                    query.update_entry(&db, &obj.id)?;
                },
                EditCommands::Method(obj) => {
                    let mut query = EditMethod::prepare(obj);
                    query.update_entry(&db, &obj.id)?;
                },
                EditCommands::Item(obj) => {
                    let mut query = EditItem::prepare(obj);
                    query.update_entry(&db, &obj.id)?;
                },
            },
            Commands::Delete(arg) => match arg {
                DeleteCommands::Company(obj) => {
                    let query = DeleteCompany::prepare(obj, &obj.id);
                    query.execute(&db)?;
                },
                DeleteCommands::Client(obj) => {
                    let query = DeleteClient::prepare(obj, &obj.id);
                    query.execute(&db)?;
                },
                DeleteCommands::Terms(obj) => {
                    let query = DeleteTerms::prepare(obj, &obj.id);
                    query.execute(&db)?;
                },
                DeleteCommands::Method(obj) => {
                    let query = DeleteMethod::prepare(obj, &obj.id);
                    query.execute(&db)?;
                },
                DeleteCommands::Item(obj) => {
                    let query = DeleteItem::prepare(obj, &obj.id);
                    query.execute(&db)?;
                },
            },
            Commands::Generate(gen) => match gen {
                GenerateCommands::Template(obj) => {
                    GenerateTemplate::generate(obj, &db)?;
                },
                GenerateCommands::Invoice(obj) => {
                    println!("{:?}, {:?}", gen, obj);
                }
            }
        }
        Ok(())
    }
}
