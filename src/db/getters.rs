use crate::db::cached::CachedStmt;
use crate::models::company::Company;
use crate::models::contact::Contact;
use anyhow::Result;

pub trait GetCompany {
    fn get_company(&self, id: &i64) -> Result<Company> {
        let query = CachedStmt {
            table: "company".to_string(),
            query: format!("SELECT * FROM company WHERE id = {} ORDER BY id", id),
            params:[].to_vec(),
        };
        let mut stmt = &self.prepare(&query.query)?;
        let company = stmt.query_map([], |row| Ok(Company {
            id: row.get(0)?,
            name: row.get(1)?,
            logo: row.get(2)?,
            contact: Contact {
                phone: row.get(3)?,
                email: row.get(4)?,
                addr1: row.get(5)?,
                addr2: row.get(6)?,
                city: row.get(7)?,
                state: row.get(8)?,
                zip: row.get(9)?,
            },
        }))?;
        Ok(company)
    }
}

