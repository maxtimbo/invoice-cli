pub mod cli;
pub mod db;
pub mod models;
pub mod commands;
pub mod render;
pub use commands::Paths;
pub use db::InvoiceDB;
pub use cli::Cli;
pub use render::TemplateEngine;

use anyhow::Result;

fn main() -> Result<()> {
    let paths = Paths::init()?;
    let mut db = InvoiceDB::open(paths.db)?;
    let renerer = TemplateEngine::new(&paths.templates)?;
    Cli::to_cmd(&mut db)?;
    Ok(())
}
