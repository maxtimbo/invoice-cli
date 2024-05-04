use crate::models::contact::Contact;
use base64::{engine::general_purpose::STANDARD, Engine};
use infer;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Company {
    pub id: i64,
    pub name: String,
    pub logo: Option<Vec<u8>>,
    pub contact: Contact,
}

impl fmt::Display for Company {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID:\t\t{}\n", self.id)?;
        write!(f, "Name:\t\t{}\n", self.name)?;
        write!(f, "Has Logo:\t{}\n", self.logo.is_some())?;
        write!(f, "Contact Information:\n{}", self.contact)
    }
}

impl Serialize for Company {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Company", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;

        if let Some(ref logo_data) = self.logo {
            if let Some(kind) = infer::get(logo_data) {
                let mime_type = kind.mime_type();
                let logo_data_uri =
                    format!("data:{};base64,{}", mime_type, STANDARD.encode(logo_data));
                state.serialize_field("logo", &logo_data_uri)?;
            } else {
                state.serialize_field("logo", &None::<String>)?;
            }
        } else {
            state.serialize_field("logo", &None::<String>)?;
        }
        state.serialize_field("contact", &self.contact)?;
        state.end()
    }
}
