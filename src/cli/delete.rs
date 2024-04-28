use crate::db::prepare::{PrepDelete, TableName};

use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum DeleteCommands {
    Company(DeleteCompany),
    Client(DeleteClient),
    Terms(DeleteTerms),
    Method(DeleteMethod),
    Item(DeleteItem),
    Template(DeleteTemplate),
}

#[derive(Debug, Args)]
pub struct DeleteCompany {
    pub id: i64,
}

#[derive(Debug, Args)]
pub struct DeleteClient{
    pub id: i64,
}

#[derive(Debug, Args)]
pub struct DeleteTerms {
    pub id: i64,
}

#[derive(Debug, Args)]
pub struct DeleteMethod {
    pub id: i64,
}

#[derive(Debug, Args)]
pub struct DeleteItem {
    pub id: i64,
}
#[derive(Debug, Args)]
pub struct DeleteTemplate{
    pub id: i64,
}

impl PrepDelete for DeleteCompany {}
impl PrepDelete for DeleteClient {}
impl PrepDelete for DeleteTerms {}
impl PrepDelete for DeleteMethod {}
impl PrepDelete for DeleteItem {}
impl PrepDelete for DeleteTemplate {}

// --- TableNames ---
impl TableName for DeleteCompany {
    fn table_name(&self) -> String {
        "company".to_string()
    }
}

impl TableName for DeleteClient {
    fn table_name(&self) -> String {
        "client".to_string()
    }
}

impl TableName for DeleteTerms {
    fn table_name(&self) -> String {
        "terms".to_string()
    }
}

impl TableName for DeleteMethod {
    fn table_name(&self) -> String {
        "methods".to_string()
    }
}
impl TableName for DeleteItem {
    fn table_name(&self) -> String {
        "items".to_string()
    }
}
impl TableName for DeleteTemplate {
    fn table_name(&self) -> String {
        "template".to_string()
    }
}
