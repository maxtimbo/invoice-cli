use crate::db::cached::CachedStmt;
use crate::db::InvoiceDB;
use crate::models::TableName;

use anyhow::Result;

impl InvoiceDB {
    pub fn create_entry(&self, cache: CachedStmt) -> Result<i64> {
        let mut stmt = self.connection.prepare(&cache.query)?;
        stmt.execute(rusqlite::params_from_iter(&cache.params))?;
        let new_id = self.connection.last_insert_rowid();
        //self.print_entry(&cache.table.as_str(), &new_id)?;
        Ok(new_id)
    }
    pub fn update_entry(&self, cache: CachedStmt, id: &i64) -> Result<()> {
        let mut stmt = self.connection.prepare(&cache.query)?;
        stmt.execute(rusqlite::params_from_iter(&cache.params))?;
        self.print_entry(cache.table, &id)?;
        Ok(())
    }
    pub fn delete_entry(&self, cache: CachedStmt, id: &i64) -> Result<()> {
        let mut stmt = self.connection.prepare(&cache.query)?;
        stmt.execute(&[id])?;
        Ok(())
    }
    pub fn print_entry(&self, table: TableName, new_id: &i64) -> Result<()> {
        match table {
            TableName::Config => {}
            TableName::Company => {
                let new_entry = self.get_company(&new_id)?;
                println!("{}", new_entry);
            }
            TableName::Client => {
                let new_entry = self.get_client(&new_id)?;
                println!("{}", new_entry);
            }
            TableName::Terms => {
                let new_entry = self.get_terms(&new_id)?;
                println!("{}", new_entry);
            }
            TableName::Methods => {
                let new_entry = self.get_method(&new_id)?;
                println!("{}", new_entry);
            }
            TableName::Items => {
                let new_entry = self.get_item(&new_id)?;
                println!("{}", new_entry);
            }
            TableName::Templates => {
                let new_entry = self.get_template(&new_id)?;
                println!("{}", new_entry);
            }
            TableName::Invoices => {
                let new_entry = self.get_invoice(&new_id)?;
                println!("{}", new_entry);
            }
            //_ => todo!("Something else!"),
        }
        //match table {
        //    "company" => {
        //        let new_entry = self.get_company(&new_id)?;
        //        println!("{}", new_entry);
        //    }
        //    "client" => {
        //        let new_entry = self.get_client(&new_id)?;
        //        println!("{}", new_entry);
        //    }
        //    "terms" => {
        //        let new_entry = self.get_terms(&new_id)?;
        //        println!("{}", new_entry);
        //    }
        //    "methods" => {
        //        let new_entry = self.get_method(&new_id)?;
        //        println!("{}", new_entry);
        //    }
        //    "items" => {
        //        let new_entry = self.get_item(&new_id)?;
        //        println!("{}", new_entry);
        //    }
        //    "templates" => {
        //        let new_entry = self.get_template(&new_id)?;
        //        println!("{}", new_entry);
        //    }
        //    "invoices" => {
        //        let new_entry = self.get_invoice(&new_id)?;
        //        println!("{}", new_entry);
        //    }
        //    _ => todo!("Something else!"),
        //}
        Ok(())
    }
}
