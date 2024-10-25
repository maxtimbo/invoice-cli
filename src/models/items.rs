use std::fmt;
use std::hash::{Hash, Hasher};

use inquire::{MultiSelect, Text, InquireError};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::models::Models;
use crate::db::prepare::ModelActions;
use crate::models::{EntityDeleter, EntityUpdater};
use crate::cli::edit::EditItem;
use crate::cli::delete::DeleteItem;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Items {
    pub table: Models,
    pub id: i64,
    pub name: String,
    pub rate: Decimal,
}

impl Items {
    pub fn new() -> Self {
        Self {
            table: Models::Items,
            id: -1,
            name: String::new(),
            rate: Decimal::new(-1,0),
        }
    }
}

impl fmt::Display for Items {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (Rate: {})", self.name, self.rate)
    }
}

impl EntityDeleter<Items> for Items {
    type Output = DeleteItem;
    fn delete(&self) -> Result<Self::Output, anyhow::Error> {
        Ok(DeleteItem { id: self.id })
    }
}

impl EntityUpdater<Items> for Items {
    type Output = EditItem;
    fn update(&self) -> Result<Self::Output, InquireError> {
        println!("{}", self);
        let fields = vec!["name", "rate"];
        let selected_fields = MultiSelect::new("Select fields to update:", fields)
            .prompt()?;
        let mut edit_item = EditItem{
            id: self.id,
            name: None,
            rate: None
        };
        for field in selected_fields {
            match field {
                "name" => {
                    let name = Text::new("Enter new name:")
                        .with_default(&self.name)
                        .prompt()?;
                    edit_item.name = Some(name);
                },
                "rate" => {
                    let rate = Text::new("Enter new rate:")
                        .with_default(&self.rate.to_string())
                        .prompt()?
                        .parse::<Decimal>()
                        .ok();
                    edit_item.rate = rate;
                },
                _ => {}
            }
        }
        Ok(edit_item)
    }
}

impl PartialEq for Items {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Items {}

impl Hash for Items {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
