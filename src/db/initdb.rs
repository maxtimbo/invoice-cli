use crate::db::InvoiceTx;

use anyhow::{Context, Result};

impl<'conn> InvoiceTx<'conn> {
    pub fn initdb(&self) -> Result<()> {
        self.tx.execute(
            "CREATE TABLE IF NOT EXISTS company (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                logo BLOB,
                phone TEXT,
                email TEXT,
                addr1 TEXT,
                addr2 TEXT,
                city TEXT,
                state TEXT,
                zip TEXT
            )", []).context("failed to create company table")?;
        self.tx.execute(
            "CREATE TABLE IF NOT EXISTS client (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                phone TEXT,
                email TEXT,
                addr1 TEXT,
                addr2 TEXT,
                city TEXT,
                state TEXT,
                zip TEXT
            )", []).context("failed to create client table")?;
        self.tx.execute(
            "CREATE TABLE IF NOT EXISTS item (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 name TEXT NOT NULL UNIQUE,
                 rate INTEGER
             )", []).context("failed to create item table")?;
        self.tx.execute(
            "CREATE TABLE IF NOT EXISTS terms (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 name TEXT NOT NULL UNIQUE,
                 due INTEGER NOT NULL UNIQUE
             )", []).context("failed to create terms table")?;
        self.tx.execute(
            "CREATE TABLE IF NOT EXISTS methods (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 name NOT NULL UNIQUE
             )", []).context("failed to create methods table")?;
        self.tx.execute(
            "CREATE TABLE IF NOT EXISTS templates (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                company_id INTEGER NOT NULL,
                client_id INTEGER NOT NULL,
                terms_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                methods_json TEXT NOT NULL,
                FOREIGN KEY (company_id)
                    REFERENCES company (id)
                    ON DELETE NO ACTION
                    ON UPDATE NO ACTION,
                FOREIGN KEY (client_id)
                    REFERENCES client (id)
                    ON DELETE NO ACTION
                    ON UPDATE NO ACTION,
                FOREIGN KEY (terms_id)
                    REFERENCES terms (id)
                    ON DELETE NO ACTION
                    ON UPDATE NO ACTION
            )", []).context("failed to create template table")?;
        self.tx.execute(
            "CREATE TABLE IF NOT EXISTS invoices (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 template_id INTEGER NOT NULL,
                 items_json TEXT NOT NULL,
                 FOREIGN KEY (template_id)
                     REFERENCES template (id)
                     ON DELETE NO ACTION
                     ON UPDATE NO ACTION
             )", []).context("failed to create invoices table")?;

        Ok(())
    }
}

