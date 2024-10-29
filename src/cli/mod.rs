use std::io;
use clap_complete::{generate, Generator, Shell};
use anyhow::Result;
use clap::{Parser, Subcommand, Command, CommandFactory};

use crate::cli::create::*;
use crate::cli::edit::*;
use crate::cli::list::*;
use crate::cli::delete::*;
use crate::cli::generate::*;
use crate::cli::config::configure_email;
use crate::render::TemplateEngine;
use crate::db::InvoiceDB;
//use crate::models::config::Config;

pub mod edit;
pub mod delete;
pub mod contact;
pub mod create;
mod generate;
mod list;
mod config;

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
#[command(name = "invoice-cli")]
pub struct Cli {
    #[arg(long = "generate", value_enum)]
    pub generator: Option<Shell>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Edit email configuration
    EditConfig,

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
        if let Some(generator) = cli.generator {
            let mut cmd = Cli::command();
            eprintln!("Generating competion file for {generator:?}...");
            print_completions(generator, &mut cmd);
        }
        if let Some(commands) = cli.command { 
            match commands {
                Commands::EditConfig => {
                    configure_email(&db)?;
                }
                Commands::Create(create) => {
                    handle_create(&create, &db)?;
                }
                Commands::List(flags) => {
                    handle_list(&flags, &db)?;
                }
                Commands::Edit(edit) => {
                    handle_edit(&edit, &db)?;
                }
                Commands::Delete(arg) => {
                    handle_delete(&arg, &db)?;
                }
                Commands::Generate(gen) => {
                    handle_generate(&gen, &db, &renderer)?;
                }
            }
        }
        Ok(())
    }
}
