use anyhow::Result;

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

fn main() -> Result<()> {
    let paths = Paths::init()?;
    let mut db = InvoiceDB::open(paths.db, 2)?;
    let renderer = TemplateEngine::new(&paths.templates)?;
    Cli::to_cmd(&mut db, &renderer)?;
    Ok(())
}
