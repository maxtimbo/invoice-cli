use crate::db::InvoiceTx;

use anyhow::{Context, Result};

impl<'conn> InvoiceTx<'conn> {
    pub fn migrate01(&self) -> Result<()> {
        self.tx.execute(
            "CREATE TABLE IF NOT EXISTS invoice_backup AS
             SELECT id, template_id, date, show_methods, show_notes, notes, items_json
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
                FOREIGN KEY (tempplate_id)
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
}

                
