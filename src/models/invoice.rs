use crate::models::company::Company;
use crate::models::client::Client;
use crate::models::terms::Terms;

pub struct Invoice {
    company: Company,
    client: Client,
    terms: Terms,
}

