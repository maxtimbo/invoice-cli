use crate::db::InvoiceDB;
use crate::db::transactions::ShortList;
use crate::models::company::Company;
use crate::models::contact::Contact;
use crate::models::client::Client;
use crate::models::terms::Terms;
use crate::models::methods::Methods;
use crate::models::items::Items;
use crate::models::invoice::{Invoice, Template};
use anyhow::Result;

impl InvoiceDB {
    pub fn get_company(&self, id: &i64) -> Result<Company, rusqlite::Error> {
        let query = "SELECT * FROM company WHERE id = ?";
        let company = self.connection.query_row(query, &[id], |row| {
            Ok(Company {
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
            })
        })?;
        Ok(company)
    }
    pub fn get_client(&self, id: &i64) -> Result<Client, rusqlite::Error> {
        let query = "SELECT * FROM client WHERE id = ?";
        let client = self.connection.query_row(query, &[id], |row| {
            Ok(Client {
                id: row.get(0)?,
                name: row.get(1)?,
                contact: Contact {
                    phone: row.get(2)?,
                    email: row.get(3)?,
                    addr1: row.get(4)?,
                    addr2: row.get(5)?,
                    city: row.get(6)?,
                    state: row.get(7)?,
                    zip: row.get(8)?,
                },
            })
        })?;
        Ok(client)
    }
    pub fn get_terms(&self, id: &i64) -> Result<Terms, rusqlite::Error> {
        let query = "SELECT * FROM terms WHERE id = ?";
        let terms = self.connection.query_row(query, &[id], |row| {
            Ok(Terms {
                id: row.get(0)?,
                name: row.get(1)?,
                due: row.get(2)?,
            })
        })?;
        Ok(terms)
    }
    pub fn get_method(&self, id: &i64) -> Result<Methods, rusqlite::Error> {
        let query = "SELECT * FROM methods WHERE id = ?";
        let method = self.connection.query_row(query, &[id], |row| {
            Ok(Methods {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;
        Ok(method)
    }
    pub fn get_item(&self, id: &i64) -> Result<Items, rusqlite::Error> {
        let query = "SELECT * FROM items WHERE id = ?";
        let item = self.connection.query_row(query, &[id], |row| {
            Ok(Items {
                id: row.get(0)?,
                name: row.get(1)?,
                rate: row.get(2)?,
            })
        })?;
        Ok(item)
    }
    pub fn get_template(&self, id: &i64) -> Result<Template, rusqlite::Error> {
        let query = "SELECT * FROM templates WHERE id = ?";
        let template = self.connection.query_row(query, &[id], |row| {
            let company_id: i64 = row.get(2)?;
            let client_id: i64 = row.get(3)?;
            let terms_id: i64 = row.get(4)?;
            let methods_json: String = row.get(5)?;

            let method_list: Vec<i64> = serde_json::from_str(&methods_json)
                .expect("Failed to deserialize methods");

            let mut methods: Vec<Methods> = Vec::new();

            for method in method_list {
                let obj = self.get_method(&method)?;
                methods.push(obj);
            }


            Ok(Template {
                id: row.get(0)?,
                name: row.get(1)?,
                company: self.get_company(&company_id)?,
                client: self.get_client(&client_id)?,
                terms: self.get_terms(&terms_id)?,
                methods: methods,
            })
        })?;
        Ok(template)
    }
    pub fn get_invoice(&self, id: &i64) -> Result<Invoice, rusqlite::Error> {
        let query = "SELECT * FROM invoices WHERE id = ?";
        let invoice = self.connection.query_row(query, &[id], |row| {
            let template_id: i64 = row.get(1)?;
            Ok(Invoice {
                id: row.get(0)?,
                template: self.get_template(&template_id)?,
                items: row.get(2)?,
            })
        })?;
        Ok(invoice)
    }

    pub fn get_table(&self, table_name: &str) -> Result<Vec<ShortList>, rusqlite::Error> {
        let query = format!("SELECT id, name FROM {}", table_name);

        let mut stmt = self.connection.prepare(&query)?;
        let short_list_iter = stmt.query_map([], |row| {
            Ok(ShortList {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let results = short_list_iter.collect::<Result<Vec<ShortList>, rusqlite::Error>>()?;
        Ok(results)
    }

}
