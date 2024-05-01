use std::fs;

use serde::{Deserialize};
use serde_json;

use crate::cli::create::{CreateCompany, CreateClient, CreateTerms, CreateMethod, CreateItem, FromJSON};

#[derive(Deserialize, Debug)]
pub struct Entities {
    pub company: Option<Vec<CreateCompany>>,
    pub client: Option<Vec<CreateClient>>,
    pub terms: Option<Vec<CreateTerms>>,
    pub method: Option<Vec<CreateMethod>>,
    pub item: Option<Vec<CreateItem>>,
}

impl FromJSON {
    pub fn from(&self) -> Result<Entities, serde_json::Error> {
        let data = fs::read_to_string(&self.json_input).expect("Error reading file");
        let entities: Entities = serde_json::from_str(&data)?;
        Ok(entities)
    }
}
