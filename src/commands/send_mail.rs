use std::fs;
use std::path::PathBuf;

use mail_send::{SmtpClientBuilder, Credentials};
use mail_builder::MessageBuilder;
use mail_builder::headers::address::Address;

use crate::models::config::Config;
use crate::models::invoice::{PaidStatus, Invoice};

impl Config {
    pub async fn send_mail(&self, output: (String, PathBuf, Invoice)) -> Result<(), anyhow::Error> {
        println!("SEND MAIL CALLED");
        let recipients: Vec<String> = output.2.template.client.contact.email.as_ref()
            .map(|emails|
                emails.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            )
            .unwrap_or_default();
        let subject = match &output.2.attributes.status {
            PaidStatus::Waiting => format!("{} {} - {}", output.2.attributes.stage, output.2.id, output.2.issue_date()),
            PaidStatus::PastDue => format!("PAST DUE:{} {} - {}", output.2.attributes.stage, output.2.id, output.2.issue_date()),
            PaidStatus::Paid { date, .. } => format!("PAID:{} {} - {}", output.2.attributes.stage, output.2.id, date),
            PaidStatus::Failed { date } => format!("FAILED:{} {} - {}", output.2.attributes.stage, output.2.id, date),
            PaidStatus::Refunded { date } => format!("REFUNDED:{} {} - {}", output.2.attributes.stage, output.2.id, date),
        };
        println!("{}", subject);
        //let subject = format!("{} {} - {}", output.2.attributes.stage, output.2.id, output.2.issue_date());
        let message = MessageBuilder::new()
            .from(Address::new_address(String::new().into(), &self.fromname))
            .to(recipients)
            .subject(subject)
            .html_body(output.0)
            .text_body(format!("{:?}", output.2))
            .attachment("application/pdf", output.1.file_name().unwrap_or_default().to_string_lossy(), fs::read(&output.1)?);
        let result = SmtpClientBuilder::new(&self.smtp_server, self.port)
            .implicit_tls(self.tls)
            .credentials(Credentials::new(&self.username.clone(), &self.password.clone()))
            .connect()
            .await
            .unwrap()
            .send(message)
            .await;
        match result {
            Ok(_) => println!("email sent"),
            Err(e) => eprintln!("error {:?}", e)
        }
        Ok(())
    }
}


