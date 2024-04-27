use crate::models::company::Company;
use crate::models::client::Client;
use crate::models::terms::Terms;

pub struct Template {
    pub id: i64,
    pub name: String,
    pub company: Company,
    pub client: Client,
    pub terms: Terms,
    pub methods: String,
}

pub struct Invoice {
    template: Template,
    items: String,
}
