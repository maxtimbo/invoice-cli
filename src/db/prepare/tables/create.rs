use crate::db::prepare::TableName;
use crate::cli::create::*;

impl TableName for CreateCompany {
    fn table_name(&self) -> String {
        "company".to_string()
    }
}

impl TableName for CreateClient {
    fn table_name(&self) -> String {
        "client".to_string()
    }
}

impl TableName for CreateTerms {
    fn table_name(&self) -> String {
        "terms".to_string()
    }
}

impl TableName for CreateMethod {
    fn table_name(&self) -> String {
        "methods".to_string()
    }
}
impl TableName for CreateItem {
    fn table_name(&self) -> String {
        "items".to_string()
    }
}

impl TableName for CreateTemplate {
    fn table_name(&self) -> String {
        "templates".to_string()
    }
}

impl TableName for CreateInvoice {
    fn table_name(&self) -> String {
        "invoices".to_string()
    }
}

