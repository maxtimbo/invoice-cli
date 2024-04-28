use crate::models::company::Company;
use crate::models::client::Client;
use crate::models::terms::Terms;
use crate::models::methods::Methods;
use serde::{Serialize, Deserialize};

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

pub struct Invoice {
    pub id: i64,
    pub template: Template,
    pub items: String,
}

impl Invoice {
    pub fn display(&self) {
        self.template.display();
        println!("Invoice\n\
            ~~~~~~~~~~~~~\n\
            id:\t\t{}\n\
            items:\t\t{}\n",
            self.id,
            self.items);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub item: i64,
    pub quantity: i64,
}
