use std::collections::HashMap;
use std::env;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct EmailReporter {
    to: String,
    from: String,
    subject: String,
    body: String,
    smtp_username: String,
    smtp_password: String,
    reported_status: HashMap<String, bool>
}

impl EmailReporter {
    pub fn new(
        mut to: Option<String>,
        mut from: Option<String>,
        mut subject: Option<String>,
        mut body: Option<String>,
        mut smtp_username: Option<String>,
        mut smtp_password: Option<String>
    ) -> EmailReporter {
        if let None = from {
            from = Some(env::var("REPORTER_MAIL_FROM")
                .expect("REPORTER_MAIL_FROM env not set!"));
        }

        if let None = to {
            to = Some(env::var("REPORTER_MAIL_TO")
                .expect("REPORTER_MAIL_TO env not set!"));
        }

        if let None = subject {
            subject = Some(env::var("REPORTER_MAIL_SUBJECT")
                .expect("REPORTER_MAIL_SUBJECT env not set!"));
        }

        if let None = body {
            body = Some(String::from("Error world!"));
        }

        if let None = smtp_username {
            smtp_username = Some(env::var("REPORTER_MAIL_SMTP_USERNAME")
                .expect("REPORTER_MAIL_SMTP_USERNAME env not set!"));
        }

        if let None = smtp_password {
            smtp_password = Some(env::var("REPORTER_MAIL_SMTP_PASSWORD")
                .expect("REPORTER_MAIL_SMTP_PASSWORD env not set!"));
        }

        let mut reported_status = HashMap::new();
        let reported_day: String = chrono::Local::now().format("[%Y-%m-%d]").to_string();
        reported_status.insert(reported_day, false);

        EmailReporter {
            to: to.unwrap(),
            from: from.unwrap(),
            subject: subject.unwrap(),
            body: body.unwrap(),
            smtp_username: smtp_username.unwrap(),
            smtp_password: smtp_password.unwrap(),
            reported_status
        }
    }

    fn already_reported(&mut self) -> bool {
        let reported_day: String = chrono::Local::now().format("[%Y-%m-%d]").to_string();
        if self.reported_status.contains_key(&reported_day) {
            return *self.reported_status.get(&reported_day).unwrap();
        }

        false
    }

    pub fn report(&mut self) {
        if self.already_reported() {
            return;
        }

        let email = Message::builder()
            .from(self.from.parse().unwrap())
            .to(self.to.parse().unwrap())
            .subject(&self.subject[..])
            .body(String::from(&self.body[..]))
            .unwrap();

        let creds = Credentials::new(self.smtp_username.to_string(), self.smtp_password.to_string());

        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => log::info!("Email sent successfully!"),
            Err(e) => log::info!("Could not send email: {:?}", e),
        }

        let reported_day: String = chrono::Local::now().format("[%Y-%m-%d]").to_string();
        self.reported_status.insert(reported_day, true);
    }
}
