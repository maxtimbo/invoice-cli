use crate::db::prepare::TableName;
use crate::cli::edit::*;
use crate::models::config::Config;

impl TableName for Config {
    fn table_name(&self) -> String {
        "email_config".to_string()
    }
}

impl TableName for EditCompany {
    fn table_name(&self) -> String {
        "company".to_string()
    }
}

impl TableName for EditClient {
    fn table_name(&self) -> String {
        "client".to_string()
    }
}

impl TableName for EditTerms {
    fn table_name(&self) -> String {
        "terms".to_string()
    }
}

impl TableName for EditMethod {
    fn table_name(&self) -> String {
        "methods".to_string()
    }
}

impl TableName for EditItem {
    fn table_name(&self) -> String {
        "items".to_string()
    }
}

impl TableName for EditTemplate {
    fn table_name(&self) -> String {
        "templates".to_string()
    }
}

impl TableName for EditInvoice {
    fn table_name(&self) -> String {
        "invoices".to_string()
    }
}
