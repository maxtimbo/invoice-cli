use crate::db::InvoiceDB;
use crate::db::cached::CachedStmt;

use anyhow::Result;

pub struct ShortList {
    pub id: i64,
    pub name: String,
}

impl InvoiceDB {
    pub fn create_entry(&self, cache: CachedStmt) -> Result<i64> {
        let mut stmt = self.connection.prepare(&cache.query)?;
        stmt.execute(rusqlite::params_from_iter(&cache.params))?;
        let new_id = self.connection.last_insert_rowid();
        self.print_entry(&cache.table.as_str(), &new_id)?;
        Ok(new_id)
    }
    pub fn update_entry(&self, cache: CachedStmt, id: &i64) -> Result<()> {
        let mut stmt = self.connection.prepare(&cache.query)?;
        stmt.execute(rusqlite::params_from_iter(&cache.params))?;
        self.print_entry(&cache.table.as_str(), &id)?;
        Ok(())
    }
    pub fn delete_entry(&self, cache: CachedStmt, id: &i64) -> Result<()> {
        let mut stmt = self.connection.prepare(&cache.query)?;
        stmt.execute(&[id])?;
        Ok(())
    }
    pub fn print_entry(&self, table: &str, new_id: &i64) -> Result<()> {
        match table {
            "company" => {
                let new_entry = self.get_company(&new_id)?;
                new_entry.display();
            },
            "client" => {
                let new_entry = self.get_client(&new_id)?;
                new_entry.display();
            },
            "terms" => {
                let new_entry = self.get_terms(&new_id)?;
                new_entry.display();
            },
            "methods" => {
                let new_entry = self.get_method(&new_id)?;
                new_entry.display();
            },
            "items" => {
                let new_entry = self.get_item(&new_id)?;
                new_entry.display();
            },
            "templates" => {
                let new_entry = self.get_template(&new_id)?;
                new_entry.display();
            },
            "invoices" => {
                let new_entry = self.get_invoice(&new_id)?;
                new_entry.display();
            },
            _ => todo!("Something else!"),
        }
        Ok(())
    }
}
