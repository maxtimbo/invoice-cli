use std::fmt;

use inquire::{MultiSelect, InquireError, Text};
use serde::{Deserialize, Serialize};

use crate::models::client::Client;
use crate::models::company::Company;
use crate::models::methods::Methods;
use crate::models::terms::Terms;
use crate::models::{EntityDeleter, EntityUpdater};

use crate::cli::delete::DeleteTemplate;
use crate::cli::edit::EditTemplate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub id: i64,
    pub name: String,
    pub company: Company,
    pub client: Client,
    pub terms: Terms,
    pub methods: Vec<Methods>,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID:\t\t{}\n", self.id)?;
        write!(f, "Name:\t\t{}\n\n", self.name)?;
        write!(f, "Company Information:\n{}\n", self.company)?;
        write!(f, "Client Information:\n{}\n", self.client)?;
        write!(f, "Terms:\n")?;
        write!(f, "{}\n", self.terms)?;
        write!(f, "Payment Methods:\n")?;
        for method in &self.methods {
            write!(f, "{}\n", method)?;
        }
        Ok(())
    }
}

impl EntityDeleter<Template> for Template {
    type Output = DeleteTemplate;
    fn delete(&self) -> Result<Self::Output, anyhow::Error> {
        Ok(DeleteTemplate { id: self.id })
    }
}

impl EntityUpdater<Template> for Template {
    type Output = EditTemplate;
    fn update(&self) -> Result<Self::Output, InquireError> {
        println!("{}", self);
        let fields = vec!["name", "company", "client", "terms", "methods"];
        let mut edit_template = EditTemplate {
            id: self.id,
            name: None,
            company: None,
            client: None,
            terms: None,
            methods: None
        };
        let selected_fields = MultiSelect::new("Select fields to update:", fields).prompt()?;
        for field in selected_fields {
            match field {
                "name" => {
                    let name = Text::new("Enter new name:")
                        .with_default(&self.name)
                        .prompt()?;
                    edit_template.name = Some(name);
                }
                "company" => {
                    //let company_selection = EntitySelector::new(db, "company", "Select Company:", true).select_entity()?;
                    //edit_template.company = company_selection;
                }
                "client" => {
                    //let client_selection = EntitySelector::new(db, "client", "Select Client:", true).select_entity()?;
                }
                "terms" => {
                    //let terms_selection = EntitySelector::new(db, "terms", "Select Payment Terms:", true).select_entity()?;
                }
                "methods" => {
                    //let methods_selection = EntitySelector::new(db, "methods", "Select Payment Methods:", true).multi_select_entity()?;
                }
                _ => {}
            }
        }
        Ok(edit_template)
    }
}

