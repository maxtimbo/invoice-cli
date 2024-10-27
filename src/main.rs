use anyhow::Result;
use tokio;

pub use cli::Cli;
pub use commands::paths::Paths;
pub use db::InvoiceDB;
pub use render::TemplateEngine;

pub mod cli;
pub mod commands;
pub mod db;
pub mod models;
pub mod render;
pub mod validators;

#[tokio::main]
async fn main() -> Result<()> {
    let paths = Paths::init()?;
    let mut db = InvoiceDB::open(paths.db, 2)?;
    let renderer = TemplateEngine::new(&paths.templates)?;
    if let Err(e) = Cli::to_cmd(&mut db, &renderer).await {
        eprintln!("Error: {:?}", e);
        return Err(e);
    }
    Ok(())
}
