use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
#[group(required = true, multiple = false)]
pub enum ListFlags {
    Company(ListCompany),
    Client(ListClient),
    Terms(ListTerms),
    Methods(ListMethods),
    Items(ListItems),
    Templates(ListTemplates),
    Invoices(ListInvoices),
}

#[derive(Debug, Args, Default)]
pub struct ListCompany {
    pub id: Option<i64>,
}
#[derive(Debug, Args, Default)]
pub struct ListClient {
    pub id: Option<i64>,
}
#[derive(Debug, Args, Default)]
pub struct ListTerms {
    pub id: Option<i64>,
}
#[derive(Debug, Args, Default)]
pub struct ListMethods {
    pub id: Option<i64>,
}
#[derive(Debug, Args, Default)]
pub struct ListItems {
    pub id: Option<i64>,
}
#[derive(Debug, Args, Default)]
pub struct ListTemplates {
    pub id: Option<i64>,
}

#[derive(Debug, Args, Default)]
pub struct ListInvoices {
    pub id: Option<i64>,
}
