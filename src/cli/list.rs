use clap::Subcommand;

#[derive(Debug, Subcommand)]
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
