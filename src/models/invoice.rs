use crate::models::company::Company;
use crate::models::client::Client;
use crate::models::terms::Terms;

pub struct Template {
    company: Company,
    client: Client,
    terms: Terms,
    methods: String,
}

pub struct Invoice {
    template: Template,
    items: String,
}
