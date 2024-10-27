use std::fmt;

use inquire::{Text, MultiSelect, Password, Confirm, InquireError};

use crate::models::EntityUpdater;
//use crate::cli::edit::EditConfig;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub id: i64,
    pub smtp_server: String,
    pub port: u16,
    pub tls: bool,
    pub username: String,
    pub password: String,
    pub fromname: String,
}

impl Config {
    pub fn default() -> Self {
        Self {
            id: 0,
            smtp_server: "smtp.example.com".to_string(),
            port: 587,
            tls: false,
            username: "username".to_string(),
            password: String::new(),
            fromname: String::new(),
        }
    }
    pub fn create(&self) -> Result<Self, InquireError> {
        let mut edit_config = Config {
            id: self.id,
            smtp_server: self.smtp_server.clone(),
            port: self.port,
            tls: self.tls,
            username: self.username.clone(),
            password: self.password.clone(),
            fromname: self.fromname.clone(),
        };
        let smtp_server = Text::new("Enter SMTP Server:")
            .with_default(&self.smtp_server)
            .prompt()?;
        edit_config.smtp_server = smtp_server;
        let port = Text::new("Enter port number:")
            .with_default(&self.port.to_string())
            .prompt()?
            .parse::<u16>()
            .ok();
        edit_config.port = port.unwrap();
        let tls = Confirm::new("Implicit TLS?")
            .with_default(self.tls)
            .prompt()?;
        edit_config.tls = tls;
        let username = Text::new("Username:")
            .with_default(&self.username)
            .prompt()?;
        edit_config.username = username;
        let password = Password::new("Password:")
            .prompt()?;
        edit_config.password = password;
        let fromname = Text::new("From name:")
            .with_default(&self.fromname)
            .prompt()?;
        edit_config.fromname = fromname;
        Ok(edit_config)
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SMTP Server:\t\t{}\n", self.smtp_server)?;
        write!(f, "Port:\t\t\t{}\n", self.port)?;
        write!(f, "TLS:\t\t\t{}\n", self.tls)?;
        write!(f, "Username:\t\t{}\n", self.username)?;
        write!(f, "Password:\t\t{}\n", self.password)?;
        write!(f, "From name:\t\t{}\n", self.fromname)
    }
}

impl EntityUpdater<Config> for Config {
    type Output = Config;
    fn update(&self) -> Result<Self::Output, InquireError> {
        println!("{}", self);
        let fields = vec!["smtp server", "port", "tls", "username", "password", "fromname"];
        let mut edit_config = Config {
            id: self.id,
            smtp_server: self.smtp_server.clone(),
            port: self.port,
            tls: self.tls,
            username: self.username.clone(),
            password: self.password.clone(),
            fromname: self.fromname.clone(),
        };
        let selected_fields = MultiSelect::new("Select fields to update:", fields).prompt()?;
        for field in selected_fields {
            match field {
                "smtp server" => {
                    let smtp_server = Text::new("Enter SMTP Server:")
                        .with_default(&self.smtp_server)
                        .prompt()?;
                    edit_config.smtp_server = smtp_server;
                }
                "port" => {
                    let port = Text::new("Enter port number:")
                        .with_default(&self.port.to_string())
                        .prompt()?
                        .parse::<u16>()
                        .ok();
                    edit_config.port = port.unwrap();
                }
                "tls" => {
                    let tls = Confirm::new("Implicit TLS?")
                        .with_default(self.tls)
                        .prompt()?;
                    edit_config.tls = tls;
                }
                "username" => {
                    let username = Text::new("Username:")
                        .with_default(&self.username)
                        .prompt()?;
                    edit_config.username = username;
                }
                "password" => {
                    let password = Password::new("Password:")
                        .prompt()?;
                    edit_config.password = password;
                }
                "fromname" => {
                    let fromname = Text::new("From name:")
                        .with_default(&self.fromname)
                        .prompt()?;
                    edit_config.fromname = fromname;
                }
                _ => {}
            }
        }
        Ok(edit_config)
    }
}

