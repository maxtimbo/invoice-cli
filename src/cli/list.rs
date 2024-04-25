use clap::{Args, Subcommand};
use crate::db::prepare::{ListTable, ListID, TableName};
use crate::db::cached::CachedStmt;

#[derive(Debug, Subcommand)]
#[group(required = true, multiple = false)]
pub enum ListFlags {
    Company(ListCompany),
    Client(ListClient),
    Terms(ListTerms),
    Methods(ListMethods),
    Items(ListItems),
}

#[derive(Debug, Args)]
pub struct ListCompany {
    #[arg(long)]
    pub id: Option<i64>,
}

#[derive(Debug, Args)]
pub struct ListClient {
    #[arg(long)]
    pub id: Option<i64>,
}
#[derive(Debug, Args)]
pub struct ListTerms {
    #[arg(long)]
    pub id: Option<i64>,
}
#[derive(Debug, Args)]
pub struct ListMethods {
    #[arg(long)]
    pub id: Option<i64>,
}
#[derive(Debug, Args)]
pub struct ListItems {
    #[arg(long)]
    pub id: Option<i64>,
}

pub trait ListDefault: ListTable + ListID {
    fn table_or_id(&self, id: Option<i64>) -> CachedStmt {
        if let Some(id) = id {
            return self.list_long(&id);
        } else {
            return self.list_short();
        }
    }
}

impl ListDefault for ListCompany {}
impl ListDefault for ListClient {}
impl ListDefault for ListTerms {}
impl ListDefault for ListMethods {}
impl ListDefault for ListItems {}

impl ListTable for ListCompany {}
impl ListTable for ListClient {}
impl ListTable for ListTerms {}
impl ListTable for ListMethods {}
impl ListTable for ListItems {}

impl ListID for ListCompany {}
impl ListID for ListClient {}
impl ListID for ListTerms {}
impl ListID for ListMethods {}
impl ListID for ListItems {}

// --- TableNames ---
impl TableName for ListCompany {
    fn table_name(&self) -> String {
        "company".to_string()
    }
}

impl TableName for ListClient {
    fn table_name(&self) -> String {
        "client".to_string()
    }
}

impl TableName for ListTerms {
    fn table_name(&self) -> String {
        "terms".to_string()
    }
}

impl TableName for ListMethods {
    fn table_name(&self) -> String {
        "methods".to_string()
    }
}
impl TableName for ListItems {
    fn table_name(&self) -> String {
        "item".to_string()
    }
}
