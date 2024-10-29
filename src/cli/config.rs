use inquire::{Confirm, Text};
use mail_send::{SmtpClientBuilder, Credentials};
use mail_builder::MessageBuilder;
use mail_builder::headers::address::Address;
use tokio::runtime::Runtime;

use crate::db::InvoiceDB;
use crate::models::config::Config;
use crate::models::EntityUpdater;
use crate::db::prepare::PrepCreateUpdate;


impl Config {
    pub async fn test_email(&self) -> Result<(), anyhow::Error> {
        let reciept = Text::new("Enter an email address for testing")
            .prompt()?;

        let message = MessageBuilder::new()
            .from(Address::new_address(String::new().into(), &self.fromname))
            .to(vec![reciept])
            .subject("Test from Invoice-CLI")
            .text_body("Test successful!");

        let result = SmtpClientBuilder::new(&self.smtp_server, self.port)
            .implicit_tls(self.tls)
            .credentials(Credentials::new(&self.username.clone(), &self.password.clone()))
            .connect()
            .await
            .unwrap()
            .send(message)
            .await;

        match result {
            Ok(_) => println!("success"),
            Err(e) => eprintln!("error {:?}", e)
        }
        Ok(())
    }
}

pub fn configure_email(db: &InvoiceDB) -> Result<(), anyhow::Error> {
    let config = match db.get_config() {
        Ok(config) => {
            println!("Adjust email configuration");
            db.create_entry(config.update()?.prepare())?;
            db.get_config()?
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            println!("Email configuration not setup yet...");
            let config = Config::default();
            db.create_entry(config.create()?.prepare())?;
            db.get_config()?
        }
        Err(e) => return Err(anyhow::Error::new(e)),
    };
    if Confirm::new("Test configuration?").prompt()? {
        let result = Runtime::new()?.block_on(config.test_email());
        if let Err(e) = result {
            eprintln!("Email test failed: {:?}", e);
        }
    }
    Ok(())
}
