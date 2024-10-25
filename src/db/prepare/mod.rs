use rusqlite::types::Value;
use rusqlite::{params, params_from_iter};

use crate::db::InvoiceDB;
use crate::db::cached::CachedStmt;
use crate::models::Models;

//pub mod tables;
pub mod fields;
pub mod values;
pub mod prep;

pub enum Crud {
    Create {
        table: Models,
        columns: Vec<String>,
        values: Vec<Value>,
    },
    ListAll {
        table: Models,
    },
    Read {
        table: Models,
        id: i64,
    },
    Update {
        table: Models,
        id: i64,
        columns: Vec<String>,
        values: Vec<Value>,
    },
    Delete {
        table: Models,
        id: i64,
    },
}

impl Crud {
    pub fn to_query(&self) -> Result<String, anyhow::Error> {
        match self {
            Crud::Create { table, columns, values } => {
                if columns.is_empty() || values.is_empty() {
                    return Err(anyhow::anyhow!("Columns or values cannot be empty"));
                }
                let placeholders = columns.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
                let columns_str = columns.join(", ");
                Ok(format!("INSERT INTO {} ({}) VALUES ({})", table.table_name(), columns_str, placeholders))
            }
            Crud::Update { table, columns, values, .. } => {
                if columns.is_empty() || values.is_empty() {
                    return Err(anyhow::anyhow!("Columns or values cannot be empty"));
                }
                let updates = columns.iter().map(|col| format!("{} = ?", col)).collect::<Vec<_>>().join(", ");
                Ok(format!("UPDATE {} SET {} WHERE id = ?", table.table_name(), updates))
            }
            Crud::ListAll { table } => {
                Ok(format!("SELECT * FROM {} ORDER BY id", table.table_name()))
            }
            Crud::Read { table, .. } => {
                Ok(format!("SELECT * FROM {} WHERE id = ? ORDER BY id", table.table_name()))
            }
            Crud::Delete { table, .. } => {
                Ok(format!("DELETE FROM {} WHERE id = ?", table.table_name()))
            }
        }
    }
    pub fn execute(&self, db: &InvoiceDB) -> Result<usize, anyhow::Error> {
        let query = self.to_query()?;
        let mut stmt = db.connection.prepare(&query)?;

        match self {
            Crud::Create { values, .. } => Ok(stmt.execute(params_from_iter(values))?),
            Crud::Update { values, .. } => Ok(stmt.execute(params_from_iter(values))?),
            Crud::ListAll { .. } => Ok(stmt.execute(params![])?),
            Crud::Read { id, .. } => Ok(stmt.execute(params![id])?),
            Crud::Delete { id, .. } => Ok(stmt.execute(params![id])?)
        }
    }
}


pub trait ModelActions {
    fn fields(&self) -> Vec<String>;
    fn values(&self) -> Vec<Value>;
}

pub trait PrepFields {
    fn fields(&self) -> Vec<std::string::String>;
}

pub trait PrepValues {
    fn values(&self) -> Vec<rusqlite::types::Value>;
}

pub trait PrepCreate: PrepFields + PrepValues {
    fn prepare(&self) -> CachedStmt {
        let fields = self.fields();
        let placeholders = fields
            .iter()
            .map(|f| format!(":{}", f))
            .collect::<Vec<_>>()
            .join(", ");
        let columns = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table_name, columns, placeholders
        );
        CachedStmt {
            table: self.table_name,
            query: query,
            params: self.values(),
        }
    }
}

pub trait PrepUpdate: PrepFields + PrepValues {
    fn prepare(&self) -> CachedStmt {
        let mut fields = self.fields();
        let table_name = table_name;
        let id = fields.remove(0);
        let placeholders = fields
            .iter()
            .map(|f| format!(":{}", f))
            .collect::<Vec<_>>()
            .join(", ");
        let columns = fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let replacements = columns
            .split(", ")
            .zip(placeholders.split(", "))
            .map(|(col, plc)| format!("{} = {}", col, plc))
            .collect::<Vec<_>>()
            .join(", ");
        let query = format!(
            "UPDATE {} SET {} WHERE id = {}",
            table_name, replacements, id
        );
        CachedStmt {
            table: table_name,
            query: query,
            params: self.values(),
        }
    }
}

//pub trait PrepDelete {
//    fn prepare(&self) -> CachedStmt {
//        CachedStmt {
//            table: self.table_name(),
//            query: format!("DELETE FROM {} WHERE id = ?", self.table_name()),
//            params: [].to_vec(),
//        }
//    }
//}
//
//pub trait ListTable {
//    fn list_short(&self) -> CachedStmt {
//        CachedStmt {
//            table: self.table_name(),
//            query: format!("SELECT id, name FROM {} ORDER BY id", self.table_name()),
//            params: [].to_vec(),
//        }
//    }
//}
//
//pub trait ListID {
//    fn list_long(&self, id: &i64) -> CachedStmt {
//        CachedStmt {
//            table: self.table_name(),
//            query: format!(
//                "SELECT * FROM {} WHERE id = {} ORDER BY id",
//                self.table_name(),
//                id
//            ),
//            params: [].to_vec(),
//        }
//    }
//}
