use crate::db::prepare::PrepFields;
use crate::cli::edit::*;
use crate::models::invoice::PaidStatus;

impl PrepFields for EditCompany {
    fn fields(&self) -> Vec<String> {
        let mut fnames = Vec::new();
        fnames.push(self.id.to_string());
        if self.name.is_some() {
            fnames.push("name".to_string());
        }
        if self.logo.is_some() {
            fnames.push("logo".to_string());
        }
        fnames.extend(self.contact.fields());
        fnames
    }
}

impl PrepFields for EditClient {
    fn fields(&self) -> Vec<String> {
        let mut fnames = Vec::new();
        fnames.push(self.id.to_string());
        if self.name.is_some() {
            fnames.push("name".to_string());
        }
        fnames.extend(self.contact.fields());
        fnames
    }
}

impl PrepFields for EditTerms {
    fn fields(&self) -> Vec<String> {
        let mut fnames = Vec::new();
        fnames.push(self.id.to_string());
        if self.name.is_some() {
            fnames.push("name".to_string());
        }
        if self.due.is_some() {
            fnames.push("due".to_string());
        }
        fnames
    }
}

impl PrepFields for EditMethod {
    fn fields(&self) -> Vec<String> {
        let mut fnames = Vec::new();
        fnames.push(self.id.to_string());
        if self.name.is_some() {
            fnames.push("name".to_string());
        }
        if self.link.is_some() {
            fnames.push("link".to_string());
        }
        if self.qr.is_some() {
            fnames.push("qr".to_string());
        }
        fnames
    }
}

impl PrepFields for EditItem {
    fn fields(&self) -> Vec<String> {
        let mut fnames = Vec::new();
        fnames.push(self.id.to_string());
        if self.name.is_some() {
            fnames.push("name".to_string());
        }
        if self.rate.is_some() {
            fnames.push("rate".to_string());
        }
        fnames
    }
}

impl PrepFields for EditTemplate {
    fn fields(&self) -> Vec<String> {
        let mut fnames = Vec::new();
        fnames.push(self.id.to_string());
        if self.name.is_some() {
            fnames.push("name".to_string());
        }
        if self.company.is_some() {
            fnames.push("company_id".to_string());
        }
        if self.client.is_some() {
            fnames.push("client_id".to_string());
        }
        if self.terms.is_some() {
            fnames.push("terms_id".to_string());
        }
        if self.methods.is_some() {
            fnames.push("methods_json".to_string());
        }
        fnames
    }
}

impl PrepFields for EditInvoice {
    fn fields(&self) -> Vec<String> {
        let mut fnames = Vec::new();
        fnames.push(self.id.to_string());
        if self.show_methods.is_some() {
            fnames.push("show_methods".to_string());
        }
        if self.show_notes.is_some() {
            fnames.push("show_notes".to_string());
        }
        if self.stage.is_some() {
            fnames.push("stage".to_string());
        }

        if self.status.is_some() {
            fnames.push("status".to_string());
            if let Some(PaidStatus::Paid { .. }) = &self.status {
                fnames.push("status_date".to_string());
                fnames.push("status_check".to_string());
            }
            if let Some(PaidStatus::Failed { .. }) | Some(PaidStatus::Refunded { .. }) = &self.status{
                fnames.push("status_date".to_string());
            }
        }

        if self.notes.is_some() {
            fnames.push("notes".to_string());
        }
        fnames
    }
}

