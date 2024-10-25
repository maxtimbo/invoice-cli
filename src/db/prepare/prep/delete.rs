use crate::db::prepare::PrepDelete;
use crate::cli::delete::*;

impl PrepDelete for DeleteCompany {}
impl PrepDelete for DeleteClient {}
impl PrepDelete for DeleteTerms {}
impl PrepDelete for DeleteMethod {}
impl PrepDelete for DeleteItem {}
impl PrepDelete for DeleteTemplate {}
impl PrepDelete for DeleteInvoice {}
