extern crate dotenv;

use std::collections::HashMap;
use std::{thread, time::Duration};
use std::env;
use dotenv::dotenv;
use log;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use chrono;

enum ReporterKind {
    EmailReporter(EmailReporter)
}

struct Reporter {
    endpoint: String,
    reporters: Vec<ReporterKind>
}

impl Reporter {
    fn new(endpoint: String) -> Reporter {
        Reporter {
            endpoint: String::from(endpoint),
            reporters: Vec::new()
        }
    }

    fn add_reporter(&mut self, reporter: ReporterKind) {
        self.reporters.push(reporter);
    }

    fn check_status(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let status = reqwest::blocking::get(&self.endpoint)?
            .status();

        return Ok(status == 200)
    }

    fn report(&mut self) {
        loop {
            thread::sleep(Duration::from_secs(1));

            if self.check_status().unwrap() {
                log::info!("App is working :)");
        
                continue;
            }
    
            for reporter in &mut self.reporters {
                match reporter {
                    ReporterKind::EmailReporter(reporter) => reporter.report(),
                }
            }

            log::info!("App is down :(");
        }
    }
}

struct EmailReporter {
    to: String,
    from: String,
    subject: String,
    body: String,
    smtp_username: String,
    smtp_password: String,
    reported_status: HashMap<String, bool>
}

impl EmailReporter {
    fn new(
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

    fn report(&mut self) {
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

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    setup_logger()?;
    
    let mut reporter = Reporter::new(env::var("REPORTER_ENDPOINT").expect("REPORTER_ENDPOINT env not set!"));

    let email_reporter = ReporterKind::EmailReporter(EmailReporter::new(None, None, None, None, None, None));
    reporter.add_reporter(email_reporter);

    reporter.report();

    Ok(())
}
