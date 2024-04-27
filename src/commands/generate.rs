use crate::cli::list::*;
use inquire::{Text, validator::{StringValidator, Validation}};

impl GenerateTemplate {
    fn generate_template(&self, db: &InvoiceDB) -> Result<()> {
        let company: ListCompany = ListCompany{ id: None };
        let query = ListCompany::table_or_id(ListCompany {id: None });
        query.list(&db, company.id)?;
        Ok(())
    }
}
