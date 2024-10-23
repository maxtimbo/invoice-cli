use crate::validators::*;
use crate::cli::edit::{EditCompany, EditMethod};

impl ValidSize for EditCompany {}
impl ValidImage for EditCompany {}
impl ValidSize for EditMethod {}
impl ValidImage for EditMethod {}
