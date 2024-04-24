pub mod prepare;
pub mod cached;
pub mod transactions;
mod initdb;

use std::path::PathBuf;

use anyhow::{Context, Result, Error};
use rusqlite::{Connection, Transaction};
use directories::ProjectDirs;

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
        tx.commit()
            .map_err(|e| e.into())
    }
}

impl InvoiceDB {
    pub fn transaction(&mut self) -> Result<InvoiceTx> {
        let transaction = self.connection.transaction()?;
        let tx = InvoiceTx {
            tx: transaction,
        };

        Ok(tx)
    }

    pub fn open() -> Result<InvoiceDB> {
        let project_dirs = ProjectDirs::from("", "", "invoice-cli")
            .ok_or_else(|| Error::msg("directory not found"))?;
        let mut db_path: PathBuf = project_dirs.data_dir().into();
        std::fs::create_dir_all(&db_path)
            .with_context(|| format!("Unable to create database dir: {:?}", &db_path))?;
        db_path.push("invoice-cli.db");
        let existing_db = db_path.is_file();
        let connection = Connection::open(db_path)?;
        connection.pragma_update(None, "foreign_keys", true)
            .context("failed to enable foreign keys pragma")?;

        let mut db = InvoiceDB {
            connection: connection
        };

        if !existing_db {
            let initdb = db.transaction()?;
            initdb.initdb()
                .context("failed to create db tables")?;
            initdb.commit()?;
        }

        Ok(db)
    }
}
