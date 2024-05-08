use crate::models::contact::Contact;
use base64::{engine::general_purpose::STANDARD, Engine};
use infer;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use std::fmt;
use std::path::PathBuf;
use crate::models::{prompt_optional, EntityUpdater};
use crate::cli::edit::EditCompany;
use crate::cli::contact::Contact as cli_contact;
use inquire::{MultiSelect, Text, InquireError};

#[derive(Debug, Deserialize)]
pub struct Company {
    pub id: i64,
    pub name: String,
    pub logo: Option<Vec<u8>>,
    pub contact: Contact,
}

impl fmt::Display for Company {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID:\t\t{}\n", self.id)?;
        write!(f, "Name:\t\t{}\n", self.name)?;
        write!(f, "Has Logo:\t{}\n", self.logo.is_some())?;
        write!(f, "Contact Information:\n{}", self.contact)
    }
}

impl EntityUpdater<Company> for Company{
    type Output = EditCompany;
    fn update(&self) -> Result<Self::Output, InquireError> {
        println!("{}", self);
        let fields = vec![
            "name",
            "logo",
            "phone",
            "email",
            "addr1",
            "addr2",
            "city",
            "state",
            "zip"];
        let selected_fields = MultiSelect::new("Select fields to update:", fields)
            .prompt()?;
        let mut edit_company = EditCompany{
            id: self.id,
            name: None,
            logo: None,
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
                    edit_company.name = Some(name);
                },
                "logo" => {
                    let logo_str = Text::new("Enter new logo path:")
                        .prompt()?;
                    if logo_str.trim().eq_ignore_ascii_case("None") {
                        edit_company.logo = None;
                    } else {
                        let logo = PathBuf::from(logo_str);
                        edit_company.logo = Some(logo);
                    }
                },
                "phone" => {
                    edit_company.contact.phone = prompt_optional(
                        "Enter new phone number (type 'None' to clear):",
                        &self.contact.phone.as_deref().unwrap_or("")
                    )?;
                },
                "email" => {
                    edit_company.contact.email = prompt_optional(
                        "Enter new email address (type 'None' to clear):",
                        &self.contact.email.as_deref().unwrap_or("")
                    )?;
                },
                "addr1" => {
                    edit_company.contact.addr1 = prompt_optional(
                        "Enter new address (type 'None' to clear):",
                        &self.contact.addr1.as_deref().unwrap_or("")
                    )?;
                },
                "addr2" => {
                    edit_company.contact.addr2 = prompt_optional(
                        "Enter new additional address (type 'None' to clear):",
                        &self.contact.addr2.as_deref().unwrap_or("")
                    )?;
                },
                "city" => {
                    edit_company.contact.city = prompt_optional(
                        "Enter new city (type 'None' to clear):",
                        &self.contact.city.as_deref().unwrap_or("")
                    )?;
                },
                "state" => {
                    edit_company.contact.state = prompt_optional(
                        "Enter new state (type 'None' to clear):",
                        &self.contact.state.as_deref().unwrap_or("")
                    )?;
                },
                "zip" => {
                    edit_company.contact.zip = prompt_optional(
                        "Enter new zip (type 'None' to clear):",
                        &self.contact.zip.as_deref().unwrap_or("")
                    )?;
                },
                _ => {}
            }
        }
        Ok(edit_company)
    }
}

impl Serialize for Company {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Company", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;

        if let Some(ref logo_data) = self.logo {
            if let Some(kind) = infer::get(logo_data) {
                let mime_type = kind.mime_type();
                let logo_data_uri =
                    format!("data:{};base64,{}", mime_type, STANDARD.encode(logo_data));
                state.serialize_field("logo", &logo_data_uri)?;
            } else {
                state.serialize_field("logo", &None::<String>)?;
            }
        } else {
            state.serialize_field("logo", &None::<String>)?;
        }
        state.serialize_field("contact", &self.contact)?;
        state.end()
    }
}
