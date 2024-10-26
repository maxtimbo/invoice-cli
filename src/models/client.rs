use std::fmt;

use serde::{Deserialize, Serialize};
use inquire::{MultiSelect, Text, InquireError};

use crate::models::contact::Contact;
use crate::models::{prompt_optional, EntityUpdater, EntityDeleter};
use crate::cli::edit::EditClient;
use crate::cli::delete::DeleteClient;
use crate::cli::contact::Contact as cli_contact;

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: i64,
    pub name: String,
    pub contact: Contact,
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID:\t\t{}\n", self.id)?;
        write!(f, "Name:\t\t{}\n", self.name)?;
        write!(f, "Contact Information:\n{}", self.contact)
    }
}

impl EntityDeleter<Client> for Client {
    type Output = DeleteClient;
    fn delete(&self) -> Result<Self::Output, anyhow::Error> {
        Ok(DeleteClient { id: self.id })
    }
}

impl EntityUpdater<Client> for Client {
    type Output = EditClient;
    fn update(&self) -> Result<Self::Output, InquireError> {
        println!("{}", self);
        let fields = vec![
            "name",
            "phone",
            "email",
            "addr1",
            "addr2",
            "city",
            "state",
            "zip"];
        let selected_fields = MultiSelect::new("Select fields to update:", fields)
            .prompt()?;
        let mut edit_client = EditClient{
            id: self.id,
            name: None,
            contact: cli_contact{
                phone: None,
                email: None,
                addr1: None,
                addr2: None,
                city: None,
                state: None,
                zip: None
            }
        };
        for field in selected_fields {
            match field {
                "name" => {
                    let name = Text::new("Enter new name:")
                        .with_default(&self.name)
                        .prompt()?;
                    edit_client.name = Some(name);
                },
                "phone" => {
                    edit_client.contact.phone = prompt_optional(
                        "Enter new phone number (type 'None' to clear):",
                        &self.contact.phone.as_deref().unwrap_or("")
                    )?;
                },
                "email" => {
                    edit_client.contact.email = prompt_optional(
                        "Enter new email address (type 'None' to clear):",
                        &self.contact.email.as_deref().unwrap_or("")
                    )?;
                },
                "addr1" => {
                    edit_client.contact.addr1 = prompt_optional(
                        "Enter new address (type 'None' to clear):",
                        &self.contact.addr1.as_deref().unwrap_or("")
                    )?;
                },
                "addr2" => {
                    edit_client.contact.addr2 = prompt_optional(
                        "Enter new additional address (type 'None' to clear):",
                        &self.contact.addr2.as_deref().unwrap_or("")
                    )?;
                },
                "city" => {
                    edit_client.contact.city = prompt_optional(
                        "Enter new city (type 'None' to clear):",
                        &self.contact.city.as_deref().unwrap_or("")
                    )?;
                },
                "state" => {
                    edit_client.contact.state = prompt_optional(
                        "Enter new state (type 'None' to clear):",
                        &self.contact.state.as_deref().unwrap_or("")
                    )?;
                },
                "zip" => {
                    edit_client.contact.zip = prompt_optional(
                        "Enter new zip (type 'None' to clear):",
                        &self.contact.zip.as_deref().unwrap_or("")
                    )?;
                },
                _ => {}
            }
        }
        Ok(edit_client)
    }
}


