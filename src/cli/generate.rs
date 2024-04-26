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

    #[arg(long, short='o', required=true)]
    pub company: String,

    #[arg(long, short, required=true)]
    pub client: String,

    #[arg(long, short, required=true)]
    pub terms: String,

    #[arg(long, short, required=true)]
    pub methods: String,
}

#[derive(Debug, Args)]
pub struct GenerateInvoice {
    pub template: String,
    
    #[arg(long, short)]
    pub output: Option<PathBuf>,
}
