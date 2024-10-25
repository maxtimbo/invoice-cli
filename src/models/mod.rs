use std::fmt;

use inquire::{Text, InquireError, Editor};
use serde::{Serialize, Deserialize};

use crate::db::prepare::ModelActions;

pub mod client;
pub mod company;
pub mod contact;
pub mod invoice;
pub mod template;
pub mod items;
pub mod methods;
pub mod terms;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Models {
    Config,
    Company,
    Client,
    Terms,
    Methods,
    Items,
    Templates,
    Invoices,
}

impl Models {
    pub fn table_name(&self) -> &'static str {
        match self {
            Models::Config => "email_config",
            Models::Company => "company",
            Models::Client => "client",
            Models::Terms => "terms",
            Models::Methods => "methods",
            Models::Items => "items",
            Models::Templates => "templates",
            Models::Invoices => "invoices",
        }
    }
    pub fn create_instance(&self) -> Box<dyn ModelActions> {
        match self {
            Models::Company => Box::new(client::Client::new()),
            Models::Client => Box::new(company::Company::new()),
            Models::Terms => Box::new(terms::Terms::new()),
            Models::Methods => Box::new(methods::Methods::new()),
            Models::Items => Box::new(items::Items::new()),
            Models::Templates => Box::new(template::Template::new()),
            Models::Invoices => Box::new(invoice::Invoice::new()),
        }
    }
}


pub struct ShortList {
    pub id: i64,
    pub name: String,
}

impl fmt::Display for ShortList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {}, Name: {}", self.id, self.name)
    }
}

pub trait EntityUpdater<T> {
    type Output;
    fn update(&self) -> Result<Self::Output, InquireError>;
}

pub trait EntityDeleter<T> {
    type Output;
    fn delete(&self) -> Result<Self::Output, anyhow::Error>;
}

pub fn prompt_optional(prompt: &str, default: &str) -> Result<Option<String>, InquireError> {
    let input = Text::new(prompt)
        .with_default(default)
        .prompt()?;

    if input.trim().eq_ignore_ascii_case("None") {
        Ok(None)
    } else {
        Ok(Some(input))
    }
}

pub fn editor_optional(prompt: &str, default: &str) -> Result<Option<String>, InquireError> {
    let input = Editor::new(prompt)
        .with_help_message("Use standard markdown syntax")
        .with_file_extension("md")
        .with_predefined_text(default)
        .prompt()?;

    if input.trim().eq_ignore_ascii_case("None") {
        Ok(None)
    } else {
        Ok(Some(input))
    }
}
