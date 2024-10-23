use crate::validators::*;
use crate::cli::create::{CreateCompany, CreateMethod};

impl ValidSize for CreateCompany {}
impl ValidImage for CreateCompany {}

impl ValidSize for CreateMethod {}
impl ValidImage for CreateMethod {}
