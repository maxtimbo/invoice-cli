use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use base64::{engine::general_purpose::STANDARD, Engine};
use infer;
use std::fmt;
use std::path::PathBuf;
use crate::models::{prompt_optional, EntityDeleter, EntityUpdater};
use crate::cli::edit::EditMethod;
use crate::cli::delete::DeleteMethod;
use inquire::{Text, InquireError, MultiSelect};

#[derive(Debug, Deserialize)]
pub struct Methods {
    pub id: i64,
    pub name: String,
    pub link: Option<String>,
    pub qr: Option<Vec<u8>>,
}

impl fmt::Display for Methods {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID:\t\t{}\n", self.id)?;
        write!(f, "Name:\t\t{}\n", self.name)?;
        if let Some(ref link) = self.link {
            write!(f, "Link:\t\t{}\n", link)?;
        } else {
            write!(f, "Link:\t\tNone\n")?;
        }
        write!(f, "Has QR:\t\t{}", self.qr.is_some())
    }
}

impl EntityDeleter<Methods> for Methods {
    type Output = DeleteMethod;
    fn delete(&self) -> Result<Self::Output, anyhow::Error> {
        Ok(DeleteMethod { id: self.id })
    }
}

impl EntityUpdater<Methods> for Methods {
    type Output = EditMethod;
    fn update(&self) -> Result<Self::Output, InquireError> {
        println!("{}", self);
        let fields = vec!["name", "link", "qr"];
        let mut edit_method = EditMethod{
            id: self.id,
            name: None,
            link: None,
            qr: None
        };
        let selected_fields = MultiSelect::new("Select fields to update:", fields).prompt()?;
        for field in selected_fields {
            match field {
                "name" => {
                    let name = Text::new("Enter new name:")
                        .with_default(&self.name)
                        .prompt()?;
                    edit_method.name = Some(name);
                },
                "link" => {
                    edit_method.link = prompt_optional(
                        "Enter new link (type 'None' to clear):",
                        &self.link.as_deref().unwrap_or("")
                        )?;
                }
                "qr" => {
                    let qr_str = Text::new("Enter new qr code image path (type 'None' to clear):").prompt()?;
                    if qr_str.trim().eq_ignore_ascii_case("None") {
                        edit_method.qr = None;
                    } else {
                        let qr = PathBuf::from(qr_str);
                        edit_method.qr = Some(qr);
                    }
                }
                _ => {}
            }
        }
        Ok(edit_method)
    }
}

impl Serialize for Methods {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Method", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        if let Some(ref link) = self.link {
            state.serialize_field("link", link)?;
        } else {
            state.serialize_field("link", &None::<String>)?;
        }

        if let Some(ref qr_data) = self.qr {
            if let Some(kind) = infer::get(qr_data) {
                let mime_type = kind.mime_type();
                let qr_data_uri =
                    format!("data:{};base64,{}", mime_type, STANDARD.encode(qr_data));
                state.serialize_field("qr", &qr_data_uri)?;
            } else {
                state.serialize_field("qr", &None::<String>)?;
            }
        } else {
            state.serialize_field("qr", &None::<String>)?;
        }
        state.end()
    }
}


