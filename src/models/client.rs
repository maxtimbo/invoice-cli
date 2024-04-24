use crate::models::contact::Contact;

#[derive(Debug)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub contact: Contact,
}
