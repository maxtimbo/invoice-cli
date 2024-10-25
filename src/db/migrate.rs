use crate::db::InvoiceTx;

use anyhow::{Context, Result};

impl<'conn> InvoiceTx<'conn> {
    pub fn create_migration_table(&self) -> Result<()> {
        self.tx.execute(
            "CREATE TABLE IF NOT EXISTS migrations (
                version INTEGER PRIMARY KEY
            );", [])
            .context("failed to insert migrations table")?;
        Ok(())
    }
    pub fn update_migration_table(&self, version: i32) -> Result<()> {
        let stmt = format!("INSERT OR REPLACE INTO migrations (version) VALUES ({});", version);
        self.tx.execute(&stmt, []).context("failed to update version")?;
        Ok(())
    }
    pub fn migrate01(&self) -> Result<()> {
        self.tx.execute(
            "CREATE TABLE IF NOT EXISTS invoice_backup AS
             SELECT id, template_id, date, items_json
             FROM invoices;", [])
            .context("failed to create backup")?;
        self.tx.execute(
            "DROP TABLE IF EXISTS invoices;", [])
            .context("failed to delete invoices")?;
        self.tx.execute(
            "CREATE TABLE IF NOT EXISTS invoices (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                template_id INTEGER NOT NULL,
                date TEXT NOT NULL,
                show_methods INTEGER NOT NULL,
                show_notes INTEGER NOT NULL,
                stage TEXT NOT NULL,
                status TEXT NOT NULL,
                status_date TEXT,
                status_check TEXT,
                notes TEXT,
                items_json TEXT NOT NULL,
                FOREIGN KEY (template_id)
                    REFERENCES templates (id)
                    ON DELETE NO ACTION
                    ON UPDATE NO ACTION
            );", [])
            .context("failed to create new table")?;
        self.tx.execute(
            "INSERT INTO invoices (
                id, template_id, date, show_methods, show_notes, stage, status, status_date, status_check, notes, items_json)
            SELECT id, template_id, date, 1, 0, 'Invoice', 'Waiting', NULL, NULL, NULL, items_json
            FROM invoice_backup;", [])
            .context("failed to restore backup")?;

        self.tx.execute("DROP TABLE IF EXISTS invoice_backup;", []).context("failed to delete backup")?;
        Ok(())
    }
    pub fn migrate02(&self) -> Result<()> {
        self.tx.execute(
            "CREATE TABLE IF NOT EXISTS email_config (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                smtp_server TEXT NOT NULL,
                port INTEGER NOT NULL,
                tls INTEGER NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL,
                fromname TEXT NOT NULL,
                fromemail TEXT NOT NULL,
                signature TEXT NOT NULL
            );", [])
            .context("failed to create email_config")?;
        Ok(())
    }
}

                
