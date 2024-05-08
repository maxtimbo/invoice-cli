use std::collections::HashMap;
use std::fmt;

use chrono::{Duration, NaiveDate};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::models::client::Client;
use crate::models::company::Company;
use crate::models::items::Items;
use crate::models::methods::Methods;
use crate::models::terms::Terms;

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
        write!(f, "Template Terms:\n")?;
        write!(f, "{}\n", self.terms)?;
        write!(f, "Template Payment Methods:\n")?;
        for method in &self.methods {
            write!(f, "{}\n", method)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Invoice {
    pub id: i64,
    pub template: Template,
    pub date: String,
    pub items: HashMap<Items, i64>,
}

impl fmt::Display for Invoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID:\t\t{}\n", self.id)?;
        write!(f, "Date:\t\t{}\n\n", self.date)?;
        write!(f, "Template Information:\n{}\n", self.template)?;
        write!(f, "Invoice Items:\n")?;
        for (item, quantity) in &self.items {
            write!(f, " - {} - Quantity: {}\n", item, quantity)?;
        }
        Ok(())
    }
}

impl Serialize for Invoice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Invoice", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("template", &self.template)?;
        let issue_date = NaiveDate::parse_from_str(&self.date, "%Y%m%d").unwrap();

        state.serialize_field("date", &issue_date.format("%B %d, %Y").to_string())?;

        // Calculate subtotals
        let items_details: Vec<ItemDetail> = self
            .items
            .iter()
            .map(|(item, &quantity)| ItemDetail {
                name: item.name.clone(),
                rate: item.rate,
                quantity,
                subtotal: item.rate * quantity,
            })
            .collect();

        state.serialize_field("items", &items_details)?;

        // Calculate total
        let total: i64 = items_details.iter().map(|item| item.subtotal).sum();
        state.serialize_field("total", &total)?;

        // Calculate due date
        let due_date = issue_date + Duration::days(self.template.terms.due);
        state.serialize_field("due_date", &due_date.format("%B %d, %Y").to_string())?;
        state.end()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub item: i64,
    pub quantity: i64,
}

#[derive(Debug, Serialize)]
struct ItemDetail {
    name: String,
    rate: i64,
    quantity: i64,
    subtotal: i64,
}
