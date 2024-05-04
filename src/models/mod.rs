use std::fmt;

pub mod client;
pub mod company;
pub mod contact;
pub mod invoice;
pub mod items;
pub mod methods;
pub mod terms;

//enum InvoiceTables {
//    Company,
//    Client,
//    Terms,
//    Methods,
//    Items,
//    Templates,
//    Invoices,
//}

pub struct ShortList {
    pub id: i64,
    pub name: String,
}

impl fmt::Display for ShortList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {}, Name: {}", self.id, self.name)
    }
}
