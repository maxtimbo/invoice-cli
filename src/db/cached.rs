use crate::db::InvoiceDB;
use crate::models::TableName;

use anyhow::Result;

pub struct CachedStmt {
    pub table: TableName,
    pub query: String,
    pub params: Vec<rusqlite::types::Value>,
}

impl CachedStmt {
    pub fn execute(&self, conn: &InvoiceDB) -> Result<()> {
        let mut stmt = conn.connection.prepare(&self.query)?;
        stmt.execute(rusqlite::params_from_iter(&self.params))?;
        Ok(())
    }
}
