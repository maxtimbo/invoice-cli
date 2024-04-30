use tera::{Tera, Context};
use anyhow::{Result, Error};
use crate::models::invoice::Invoice;

use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new(template_path: &PathBuf) -> Result<Self> {
        let template_str = &template_path.to_str().unwrap();
        let glob_pattern = format!("{}/**/*", template_str);
        let tera = Tera::new(&glob_pattern)
            .map_err(|e| Error::msg(format!("Failed to initalize Tera: {}", e)))?;
        Ok(TemplateEngine { tera })
    }
    pub fn render(&self, invoice: &Invoice) -> Result<String> {
        let context = Context::from_serialize(invoice)
            .map_err(|e| Error::msg(format!("Context error: {}", e)))?;

        let rendered = self.tera.render("default.html", &context)
            .map_err(|e| {
                eprintln!("Detailed error: {:?}", e);
                Error::msg(format!("Template rendering error: {}", e))
            })?;
        Ok(rendered)
    }
    pub fn to_file(&self, rendered: &String, output_file: &PathBuf) -> Result<()> {
        let mut file = File::create(output_file)
            .map_err(|e| Error::msg(format!("Failed to create output file: {}", e)))?;

        file.write_all(rendered.as_bytes())
            .map_err(|e| Error::msg(format!("Failed to write output file: {}", e)))?;

        Ok(())
    }
}
