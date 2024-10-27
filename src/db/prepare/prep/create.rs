use crate::db::prepare::{PrepCreate, PrepCreateUpdate};
use crate::cli::create::*;
use crate::models::config::Config;

impl PrepCreateUpdate for Config {}
impl PrepCreate for CreateCompany {}
impl PrepCreate for CreateClient {}
impl PrepCreate for CreateTerms {}
impl PrepCreate for CreateMethod {}
impl PrepCreate for CreateItem {}
impl PrepCreate for CreateTemplate {}
impl PrepCreate for CreateInvoice {}
