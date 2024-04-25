use std::path::PathBuf;
use anyhow::{Context, Result, Error};
use directories::ProjectDirs;

#[derive(Debug)]
pub struct Paths {
    pub db: PathBuf,
    pub templates: PathBuf,
}

impl Paths {
    pub fn init() -> Result<Paths> {
        let project_dirs = ProjectDirs::from("", "", "invoice-cli")
            .ok_or_else(|| Error::msg("directory not found"))?;
        let mut db_path: PathBuf = project_dirs.data_dir().into();
        std::fs::create_dir_all(&db_path)
            .with_context(|| format!("Unable to create project root dir: {:?}", &db_path))?;
        db_path.push("invoice-cli.db");


        let mut templates: PathBuf = project_dirs.data_dir().into(); 
        templates.push("templates");
        std::fs::create_dir_all(&templates)
            .with_context(|| format!("Unable to create templates dir: {:?}", &templates))?;

        let paths = Paths {
            db: db_path,
            templates: templates,
        };
        Ok(paths)
    }
}


