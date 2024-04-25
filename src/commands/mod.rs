use std::path::PathBuf;
use anyhow::{Context, Result, Error};
use directories::ProjectDirs;

#[derive(Debug)]
pub struct Paths {
    pub db: PathBuf,
    pub templates: PathBuf,
    pub imgs: PathBuf,
}

impl Paths {
    pub fn init() -> Result<Paths> {
        let project_dirs = ProjectDirs::from("", "", "invoice-cli")
            .ok_or_else(|| Error::msg("directory not found"))?;

        // Create database path
        let mut db_path: PathBuf = project_dirs.data_dir().into();
        std::fs::create_dir_all(&db_path)
            .with_context(|| format!("Unable to create project root dir: {:?}", &db_path))?;
        db_path.push("invoice-cli.db");

        
        // Create templates path
        let mut templates: PathBuf = project_dirs.data_dir().into(); 
        templates.push("templates");
        std::fs::create_dir_all(&templates)
            .with_context(|| format!("Unable to create templates dir: {:?}", &templates))?;

        // Create defaults file if none exists
        templates.push("default.html");
        let existing_default = templates.is_file();
        if !existing_default {
            let default_html = include_str!("../templates/default.html");
            std::fs::write(&templates, default_html).expect("Unable to create default.html");
        }

        // Create images path
        let mut imgs: PathBuf = project_dirs.data_dir().into();
        imgs.push("imgs");
        std::fs::create_dir_all(&imgs).with_context(|| format!("Unable to create imgs dir: {:?}", &imgs))?;

        let paths = Paths {
            db: db_path,
            templates: templates,
            imgs: imgs,
        };
        Ok(paths)
    }
}


