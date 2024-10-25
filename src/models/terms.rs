use serde::{Deserialize, Serialize};
use std::fmt;
use inquire::{MultiSelect, Text, InquireError};

use crate::models::Models;
use crate::db::prepare::ModelActions;
use crate::models::{EntityDeleter, EntityUpdater};
use crate::cli::edit::EditTerms;
use crate::cli::delete::DeleteTerms;

#[derive(Debug, Serialize, Deserialize)]
pub struct Terms {
    pub table: Models,
    pub id: i64,
    pub name: String,
    pub due: i64,
}

impl Terms {
    pub fn new() -> Self {
        Self {
            table: Models::Terms,
            id: -1,
            name: String::new(),
            due: -1,
        }
    }
}

impl fmt::Display for Terms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {} - Name: {}, Due: {}",
            self.id, self.name, self.due
        )
    }
}

impl EntityDeleter<Terms> for Terms {
    type Output = DeleteTerms;
    fn delete(&self) -> Result<Self::Output, anyhow::Error> {
        Ok(DeleteTerms { id: self.id })
    }
}

impl EntityUpdater<Terms> for Terms {
    type Output = EditTerms;
    fn update(&self) -> Result<Self::Output, InquireError> {
        println!("{}", self);
        let fields = vec!["name", "due"];
        let selected_fields = MultiSelect::new("Select fields to update:", fields)
            .prompt()?;
        let mut edit_terms = EditTerms{
            id: self.id,
            name: None,
            due: None
        };
        for field in selected_fields {
            match field {
                "name" => {
                    let name = Text::new("Enter new name:")
                        .with_default(&self.name)
                        .prompt()?;
                    edit_terms.name = Some(name);
                },
                "due" => {
                    let due = Text::new("Enter new due date delta:")
                        .with_default(&self.due.to_string())
                        .prompt()?
                        .parse::<i64>()
                        .ok();
                    edit_terms.due = due;
                },
                _ => {}
            }
        }
        Ok(edit_terms)
    }
}
