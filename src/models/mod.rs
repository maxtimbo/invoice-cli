use std::fmt;

use inquire::{Text, InquireError};

pub mod client;
pub mod company;
pub mod contact;
pub mod invoice;
pub mod items;
pub mod methods;
pub mod terms;

//enum InvoiceTables {
//    Company,
//    Client,
//    Terms,
//    Methods,
//    Items,
//    Templates,
//    Invoices,
//}

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

