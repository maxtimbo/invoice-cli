pub mod cached;
pub mod getters;
mod initdb;
mod migrate;
pub mod prepare;
pub mod transactions;

use std::path::PathBuf;

use anyhow::{Context, Result};
use rusqlite::{Connection, Transaction, OptionalExtension};
//use directories::ProjectDirs;

#[derive(Debug)]
pub struct InvoiceDB {
    connection: Connection,
}

pub struct InvoiceTx<'conn> {
    tx: Transaction<'conn>,
}

impl<'conn> InvoiceTx<'conn> {
    pub fn commit(self) -> Result<()> {
        let tx = self.tx;
        tx.commit().map_err(|e| e.into())
    }
}

impl InvoiceDB {
    pub fn transaction(&mut self) -> Result<InvoiceTx> {
        let transaction = self.connection.transaction()?;
        let tx = InvoiceTx { tx: transaction };

        Ok(tx)
    }

    pub fn open(db_path: PathBuf, version: i32) -> Result<InvoiceDB> {
        let existing_db = db_path.is_file();
        let connection = Connection::open(db_path)?;
        connection
            .pragma_update(None, "foreign_keys", true)
            .context("failed to enable foreign keys pragma")?;

        let mut db = InvoiceDB { connection };

        if !existing_db {
            let initdb = db.transaction()?;
            initdb.initdb().context("failed to create db tables")?;
            initdb.tx
                .execute("INSERT INTO migrations (version) VALUES (1);", [])
                .context("failed to set initial migration version")?;
            initdb.commit().context("failed to commit transaction")?;
        } else {
            let current_version: Result<Option<i32>, rusqlite::Error> = db
                .connection
                .query_row(
                    "SELECT version FROM migrations LIMIT 1",
                    [],
                    |row| row.get(0)
                ).optional();
            match current_version {
                Ok(Some(vers)) if vers < version => { db.run_migrations(version)?; }
                Ok(None) => { db.run_migrations(version)?; }
                Ok(Some(_)) => {}
                Err(err) => {
                    return Err(anyhow::Error::new(err).context("failed to query migrations table"));
                }
            }
        }

        Ok(db)
    }
    pub fn run_migrations(&mut self, version: i32) -> Result<()> {
        let tx = self.transaction()?;
        tx.migrate01()?;
        tx.commit()?;

        //let tx = self.transaction()?;
        self.connection.execute("CREATE TABLE IF NOT EXISTS migrations (
            version INTEGER PRIMARY KEY);", [])
            .context("failed to insert migrations table")?;
        self.connection.execute(&format!("INSERT OR REPLACE INTO migrations (version) VALUES ({})", version), [])?;
        Ok(())
    }

}
