use std::collections::HashMap;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct EmailReporter {
    to: Option<String>,
    from: Option<String>,
    subject: Option<String>,
    body: Option<String>,
    smtp_username: Option<String>,
    smtp_password: Option<String>,
    reported_status: HashMap<String, bool>,
    report_once: bool
}

impl EmailReporter {
    pub fn new() -> EmailReporter {
        let mut reported_status = HashMap::new();
        let reported_day: String = chrono::Local::now().format("[%Y-%m-%d]").to_string();
        reported_status.insert(reported_day, false);

        EmailReporter {
            to: None,
            from: None,
            subject: None,
            body: None,
            smtp_username: None,
            smtp_password: None,
            reported_status,
            report_once: true
        }
    }

    pub fn to(&mut self, to: Option<String>) -> &mut EmailReporter {
        self.to = to;

        return self;
    }

    pub fn from(&mut self, from: Option<String>) -> &mut EmailReporter {
        self.from = from;

        return self;
    }

    pub fn subject(&mut self, subject: Option<String>) -> &mut EmailReporter {
        self.subject = subject;

        return self;
    }

    pub fn body(&mut self, body: Option<String>) -> &mut EmailReporter {
        self.body = body;

        return self;
    }

    pub fn smtp_username(&mut self, smtp_username: Option<String>) -> &mut EmailReporter {
        self.smtp_username = smtp_username;

        return self;
    }

    pub fn smtp_password(&mut self, smtp_password: Option<String>) -> &mut EmailReporter {
        self.smtp_password = smtp_password;

        return self;
    }

    fn already_reported(&mut self) -> bool {
        if self.report_once {
            return false;
        }

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
            .from(self.from.clone().unwrap().as_str().parse().unwrap())
            .to(self.to.clone().unwrap().as_str().parse().unwrap())
            .subject(self.subject.clone().unwrap())
            .body(self.body.clone().unwrap())
            .unwrap();

        let creds = Credentials::new(
            self.smtp_username.clone().unwrap().as_str().parse().unwrap(),
            self.smtp_password.clone().unwrap().as_str().parse().unwrap()
        );

        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => println!("Could not send email: {:?}", e),
        }

        let reported_day: String = chrono::Local::now().format("[%Y-%m-%d]").to_string();
        self.reported_status.insert(reported_day, true);
    }
}
