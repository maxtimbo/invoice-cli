pub mod cli;
pub use cli::Cli;
pub mod db;
pub use db::InvoiceDB;
pub mod models;
pub mod commands;
pub use commands::Paths;

use anyhow::Result;

fn main() -> Result<()> {
    let paths = Paths::init()?;
    let db = InvoiceDB::open(paths.db)?;
    Cli::to_cmd(&db)?;
    Ok(())
}
