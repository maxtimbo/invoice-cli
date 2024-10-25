use crate::db::prepare::TableName;
use crate::cli::delete::*;

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
        "templates".to_string()
    }
}
impl TableName for DeleteInvoice {
    fn table_name(&self) -> String {
        "invoices".to_string()
    }
}
