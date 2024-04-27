use crate::db::InvoiceDB;
use crate::db::cached::CachedStmt;
use crate::models::company::Company;
use crate::models::client::Client;
use crate::models::methods::Methods;
use crate::models::terms::Terms;
use crate::models::items::Items;
use crate::models::contact::Contact;
use crate::models::invoice::Template;

use anyhow::Result;

struct ShortList {
    id: i64,
    name: String,
}

impl CachedStmt {
    pub fn create_entry(&mut self, conn: &InvoiceDB) -> Result<()> {
        let mut stmt = conn.connection.prepare(&self.query)?;
        stmt.execute(rusqlite::params_from_iter(&self.params))?;
        let new_id = conn.connection.last_insert_rowid();
        self.query = format!("SELECT * FROM {} WHERE id = {} ORDER BY id", self.table, new_id);
        self.long_list(&conn)?;
        Ok(())
    }
    pub fn update_entry(&mut self, conn: &InvoiceDB, id: &i64) -> Result<()> {
        let mut stmt = conn.connection.prepare(&self.query)?;
        stmt.execute(rusqlite::params_from_iter(&self.params))?;
        self.query = format!("SELECT * FROM {} WHERE id = {} ORDER BY id", self.table, id);
        self.long_list(&conn)?;
        Ok(())
    }

    pub fn list(&self, conn: &InvoiceDB, id: Option<i64>) -> Result<()> {
        if let Some(_id) = id {
            self.long_list(&conn)?;
        } else {
            self.list_table(&conn)?;
        }
        Ok(())
    }

    pub fn list_table(&self, conn: &InvoiceDB) -> Result<Vec<i64>> {
        let mut stmt = conn.connection.prepare(&self.query)?;
        let resp = stmt.query_map([], |row| Ok(ShortList {
            id: row.get(0)?,
            name: row.get(1)?,
        }))?;
        println!("Entries for {}\n", self.table);
        let mut ids: Vec<i64> = Vec::new();
        for row in resp.flatten() {
            println!("+-----\n| id:\t{}\n| name:\t{}", row.id, row.name);
            ids.push(row.id);
        }
        println!("+-----");
        Ok(ids)
    }
    pub fn long_list(&self, conn: &InvoiceDB) -> Result<()> {
        match self.table.as_str() {
            "company" => {
                long_list_company(&conn, &self.query)?;
            },
            "client" => {
                long_list_client(&conn, &self.query)?;
            },
            "terms" => {
                long_list_terms(&conn, &self.query)?;
            },
            "methods" => {
                long_list_method(&conn, &self.query)?;
            },
            "item" => {
                long_list_item(&conn, &self.query)?;
            },
            _ => todo!("Something else!"),
        }
        Ok(())
    }
}

fn long_list_company(conn: &InvoiceDB, query: &str) -> Result<()> {
    let mut resp = conn.connection.prepare(query)?;
    let new_entry= resp.query_map([], |row| Ok(Company {
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
    for data in new_entry.flatten() {
        println!("Company\n\
            ~~~~~~~~~~~~\n\
            id:\t\t{}\n\
            name:\t\t{}\n\
            logo:\t\t\n\
            phone:\t\t{}\n\
            email:\t\t{}\n\
            addr1:\t\t{}\n\
            addr2:\t\t{}\n\
            city:\t\t{}\n\
            state:\t\t{}\n\
            zip:\t\t{}",
            data.id,
            data.name,
            //data.logo.as_deref().unwrap_or("None"),
            data.contact.phone.as_deref().unwrap_or("None"),
            data.contact.email.as_deref().unwrap_or("None"),
            data.contact.addr1.as_deref().unwrap_or("None"),
            data.contact.addr2.as_deref().unwrap_or("None"),
            data.contact.city.as_deref().unwrap_or("None"),
            data.contact.state.as_deref().unwrap_or("None"),
            data.contact.zip.as_deref().unwrap_or("None")
        );
    }
    Ok(())
}

fn long_list_client(conn: &InvoiceDB, query: &str) -> Result<()> {
    let mut resp = conn.connection.prepare(&query)?;
    let new_entry= resp.query_map([], |row| Ok(Client {
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
    }))?;
    for data in new_entry.flatten() {
        println!("Client\n\
            ~~~~~~~~~~~~\n\
            id:\t\t{}\n\
            name:\t\t{}\n\
            phone:\t\t{}\n\
            email:\t\t{}\n\
            addr1:\t\t{}\n\
            addr2:\t\t{}\n\
            city:\t\t{}\n\
            state:\t\t{}\n\
            zip:\t\t{}",
            data.id,
            data.name,
            data.contact.phone.as_deref().unwrap_or("None"),
            data.contact.email.as_deref().unwrap_or("None"),
            data.contact.addr1.as_deref().unwrap_or("None"),
            data.contact.addr2.as_deref().unwrap_or("None"),
            data.contact.city.as_deref().unwrap_or("None"),
            data.contact.state.as_deref().unwrap_or("None"),
            data.contact.zip.as_deref().unwrap_or("None")
        );
    }
    Ok(())
}

fn long_list_method(conn: &InvoiceDB, query: &str) -> Result<()> {
    let mut resp = conn.connection.prepare(&query)?;
    let new_entry = resp.query_map([], |row| Ok(Methods {
        id: row.get(0)?,
        name: row.get(1)?,
        },
    ))?;
    for data in new_entry.flatten() {
        println!("Payment Method\n\
            ~~~~~~~~~~~~\n\
            id:\t\t{}\n\
            name:\t\t{}",
            data.id,
            data.name,
        );
    }
    Ok(())
}
fn long_list_terms(conn: &InvoiceDB, query: &str) -> Result<()> {
    let mut resp = conn.connection.prepare(&query)?;
    let new_entry = resp.query_map([], |row| Ok(Terms {
        id: row.get(0)?,
        name: row.get(1)?,
        due: row.get(2)?,
        },
    ))?;
    for data in new_entry.flatten() {
        println!("Terms\n\
            ~~~~~~~~~~~~\n\
            id:\t\t{}\n\
            name:\t\t{}\n\
            due:\t\t{}",
            data.id,
            data.name,
            data.due
        );
    }
    Ok(())
}

fn long_list_item(conn: &InvoiceDB, query: &str) -> Result<()> {
    let mut resp = conn.connection.prepare(&query)?;
    let new_entry = resp.query_map([], |row| Ok(Items {
        id: row.get(0)?,
        name: row.get(1)?,
        rate: row.get(2)?,
        },
    ))?;
    for data in new_entry.flatten() {
        println!("Item\n\
            ~~~~~~~~~~~~\n\
            id:\t\t{}\n\
            name:\t\t{}\n\
            rate:\t\t{}",
            data.id,
            data.name,
            data.rate
        );
    }
    Ok(())
}

fn long_list_template(conn: &InvoiceDB, query: &str) -> Result<()> {
    let mut resp = conn.connection.prepare(&query)?;
    let new_entry = resp.query_map([], |row| Ok(Template {
        id: row.get(0)?,
        //company: long_list_company(&conn, format!("SELECT * FROM company WHERE id = {}", row.get(1))?)?,
        company: todo!("idk yet"),
        client: todo!(),
        terms: todo!(),
        //client: row.get(2)?,
        //terms: row.get(3)?,
        name: row.get(4)?,
        methods: row.get(5)?,
        },
    ));
    Ok(())
}

