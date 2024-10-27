use rust_decimal::Decimal;
use anyhow::Result;
use inquire::{MultiSelect, Text, Confirm, Select};

use crate::db::InvoiceDB;
use crate::cli::create::{CreateCommands, CreateItem, CreateTerms, handle_create};

pub struct EntitySelector<'a> {
    pub db: &'a InvoiceDB,
    pub table: &'a str,
    pub prompt: &'a str,
    pub allow_add: bool,
    add_new_str: String,
}

impl<'a> EntitySelector<'a> {
    pub fn new(db: &'a InvoiceDB, table: &'a str, prompt: &'a str, allow_add: bool) -> Self {
        let add_new_str = format!("Add new {}", table);
        EntitySelector {
            db,
            table,
            prompt,
            allow_add,
            add_new_str,
        }
    }
    pub fn multi_select_entity(&self) -> Result<Vec<i64>, anyhow::Error> {
        let mut selection = MultiSelect::new(&self.prompt, self.get_options()?).prompt()?;
        let mut add_more = self.allow_add && selection.contains(&self.add_new_str.to_string());
        selection.retain(|s| s != &self.add_new_str);

        while add_more {
            let new_entity = self.add_new_entity()?;
            selection.push(new_entity.to_string());
            add_more = Confirm::new("Add another?").prompt()?;
        }
        Ok(selection.iter()
            .filter_map(|s| s.split(" - ").next()?.parse::<i64>().ok())
            .collect())
    }

    pub fn select_entity(&self) -> Result<i64, anyhow::Error> {
        let selection = Select::new(&self.prompt, self.get_options()?)
            .prompt()
            .map(|ans| ans.split(" - ").next().unwrap().parse::<i64>().unwrap())?;
        Ok(selection)
    }

    fn get_options(&self) -> Result<Vec<String>, anyhow::Error> {
        let add_new_str = format!("Add new {}", self.table);
        let list_result = self.db.get_table(self.table)?;
        if list_result.is_empty() {
            Err(anyhow::anyhow!("The {} table is empty.", &self.table))
        } else {
            let mut options: Vec<String> = list_result
                .iter()
                .map(|sl| format!("{} - {}", sl.id, sl.name))
                .collect::<Vec<_>>();
            if self.allow_add {
                options.push(add_new_str.to_string());
            }
            Ok(options)
        }
    }

    fn add_new_entity(&self) -> Result<i64, anyhow::Error> {
        let name = Text::new(&format!("Enter new {} name:", &self.table).to_string()).prompt()?;
        match self.table {
            "company" => handle_create(&CreateCommands::Company{ name: name.clone() }, &self.db),
            "clients" => handle_create(&CreateCommands::Client{ name: name.clone() }, &self.db),
            "terms" => {
                let due = Text::new("Enter due date:")
                    .prompt()?
                    .parse::<u32>()
                    .map_err(|_| anyhow::anyhow!("Invalid number"))?;
                let new_terms = CreateCommands::Terms(CreateTerms {
                    name: name.clone(),
                    due,
                });
                handle_create(&new_terms, &self.db)
            }

            "items" => {
                let rate = Text::new("Enter item rate:")
                    .prompt()?
                    .parse::<Decimal>()
                    .map_err(|_| anyhow::anyhow!("Invalid number"))?;
                let new_item = CreateCommands::Item(CreateItem {
                    name: name.clone(),
                    rate,
                });
                handle_create(&new_item, &self.db)
            }
            "methods" => handle_create(&CreateCommands::Method{ name: name.clone() }, &self.db),
            _ => Err(anyhow::anyhow!("Unknown entity type")),
        }
    }
}
