use crate::db::prepare::PrepFields;
use crate::cli::create::*;

impl PrepFields for CreateCompany {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("name".to_string());
        if self.logo.is_some() {
            fnames.push("logo".to_string());
        }
        if let Some(contact) = &self.contact {
            fnames.extend(contact.fields());
        }
        fnames
    }
}

impl PrepFields for CreateClient {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("name".to_string());
        if let Some(contact) = &self.contact {
            fnames.extend(contact.fields());
        }
        fnames
    }
}

impl PrepFields for CreateTerms {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("name".to_string());
        fnames.push("due".to_string());
        fnames
    }
}

impl PrepFields for CreateMethod {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("name".to_string());
        if self.link.is_some() {
            fnames.push("link".to_string());
        }
        if self.qr.is_some() {
            fnames.push("qr".to_string());
        }
        fnames
    }
}

impl PrepFields for CreateItem {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("name".to_string());
        fnames.push("rate".to_string());
        fnames
    }
}

impl PrepFields for CreateTemplate {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("name".to_string());
        fnames.push("company_id".to_string());
        fnames.push("client_id".to_string());
        fnames.push("terms_id".to_string());
        fnames.push("methods_json".to_string());
        fnames
    }
}

impl PrepFields for CreateInvoice {
    fn fields(&self) -> Vec<std::string::String> {
        let mut fnames = Vec::new();
        fnames.push("template_id".to_string());
        fnames.push("date".to_string());
        fnames.push("show_methods".to_string());
        fnames.push("show_notes".to_string());
        fnames.push("stage".to_string());
        fnames.push("status".to_string());
        fnames.push("status_date".to_string());
        fnames.push("status_check".to_string());
        fnames.push("notes".to_string());
        fnames.push("items_json".to_string());
        fnames
    }
}

