pub mod cli;
pub use cli::Cli;
pub mod db;
pub use db::InvoiceDB;
pub mod models;

use anyhow::Result;

fn main() -> Result<()> {
    let db = InvoiceDB::open()?;
    Cli::to_cmd(&db)?;
    Ok(())
}
