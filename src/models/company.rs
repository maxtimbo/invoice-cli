use crate::models::contact::Contact;

#[derive(Debug)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub logo: Option<String>,
    pub contact: Contact,
}
