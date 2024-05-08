use serde::{Deserialize, Serialize};
use std::fmt;
use crate::models::EntityUpdater;
use crate::cli::edit::EditMethod;
use inquire::{Text, InquireError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Methods {
    pub id: i64,
    pub name: String,
}

impl fmt::Display for Methods {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {} - Name: {}", self.id, self.name)
    }
}

impl EntityUpdater<Methods> for Methods {
    type Output = EditMethod;
    fn update(&self) -> Result<Self::Output, InquireError> {
        println!("{}", self);
        let mut edit_method = EditMethod{
            id: self.id,
            name: None
        };
        let name = Text::new("Enter new name:")
            .with_default(&self.name)
            .prompt()?;
        edit_method.name = Some(name);
        Ok(edit_method)
    }
}
