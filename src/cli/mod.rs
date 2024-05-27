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
use crate::db::InvoiceDB;

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
            Commands::Create(create) => {
                handle_create(create, &db)?;
            }
            Commands::List(flags) => {
                handle_list(flags, &db)?;
            },
            Commands::Edit(edit) => {
                handle_edit(edit, &db)?;
            },
            Commands::Delete(arg) => {
                handle_delete(arg, &db)?;
            },
            Commands::Generate(gen) => {
                handle_generate(gen, &db, &renderer)?;
            },
        }
        Ok(())
    }
}
