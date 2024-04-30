use crate::models::company::Company;
use crate::models::client::Client;
use crate::models::terms::Terms;
use crate::models::methods::Methods;
use crate::models::items::Items;
use serde::{Serialize, Serializer, ser::SerializeStruct, Deserialize};
use chrono::{NaiveDate, Duration};

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub id: i64,
    pub name: String,
    pub company: Company,
    pub client: Client,
    pub terms: Terms,
    pub methods: Vec<Methods>,
}

impl Template {
    pub fn display(&self) {
        println!("Template\n\
            ~~~~~~~~~~~~~~\n\
            id:\t\t{}\n\
            name:\t\t{}\n",
            self.id,
            self.name);
        self.company.display();
        println!("");
        self.client.display();
        println!("");
        self.terms.display();
        println!("");
        for method in &self.methods {
            method.display();
        }
    }
}

#[derive(Debug)]
pub struct Invoice {
    pub id: i64,
    pub template: Template,
    pub date: String,
    pub items: HashMap<Items, i64>,
}

impl Invoice {
    pub fn display(&self) {
        self.template.display();
        println!("");
        println!("Invoice\n\
            ~~~~~~~~~~~~~\n\
            id:\t\t{}\n\
            date:\t\t{}\n\
            items:",
            self.id,
            self.date);
        for (item, quantity) in &self.items {
            println!(" - {} - Quantity: {}", item, quantity);
        }
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
            let items_details: Vec<ItemDetail> = self.items.iter()
                .map(|(item, &quantity)| ItemDetail {
                    name: item.name.clone(),
                    rate: item.rate,
                    quantity,
                    subtotal: (item.rate as i64) * quantity,
                })
                .collect();

            state.serialize_field("items", &items_details)?;

            // Calculate total
            let total: i64 = items_details.iter()
                .map(|item| item.subtotal)
                .sum();
            state.serialize_field("total", &total)?;

            // Calculate due date
            let due_date = issue_date + Duration::days(self.template.terms.due);
            state.serialize_field("due_date", &due_date.format("%B %d, %Y").to_string())?;
            //let items: HashMap<String, i64> = self.items.iter()
            //    .map(|(key, &value)| (format!("{}: {}", key.name, key.rate), value))
            //    .collect();
            //state.serialize_field("items", &items)?;
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
    rate: i32,
    quantity: i64,
    subtotal: i64,
}
