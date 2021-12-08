mod reporter;

use std::env;
use dotenv::dotenv;
use log;
use chrono;
use std::{thread, time::Duration};
use crate::reporter::{Reporter, ReporterKind, EmailReporter};

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

fn check_status(endpoint: &String) -> Result<bool, reqwest::Error> {
    let status = reqwest::blocking::get(endpoint)?
        .status();

    return Ok(status == 200)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    setup_logger()?;
    
    let mut reporter = Reporter::new();

    let mut email_reporter = EmailReporter::new();
    email_reporter
        .to(Some(env::var("REPORTER_MAIL_TO").expect("REPORTER_MAIL_TO env not set!")))
        .from(Some(env::var("REPORTER_MAIL_FROM").expect("REPORTER_MAIL_FROM env not set!")))
        .subject(Some(env::var("REPORTER_MAIL_SUBJECT").expect("REPORTER_MAIL_SUBJECT env not set!")))
        .smtp_username(Some(env::var("REPORTER_MAIL_SMTP_USERNAME").expect("REPORTER_MAIL_SMTP_USERNAME env not set!")))
        .smtp_password(Some(env::var("REPORTER_MAIL_SMTP_PASSWORD").expect("REPORTER_MAIL_SMTP_PASSWORD env not set!")))
        .body(Some(String::from("Error world!")));
        
    reporter.add_reporter(
        ReporterKind::EmailReporter(email_reporter)
    );

    loop {
        thread::sleep(Duration::from_secs(1));

        let endpoint = env::var("REPORTER_ENDPOINT").expect("REPORTER_ENDPOINT env not set!");

        if check_status(&endpoint).ok() != None {
            log::info!("App is working :)");
    
            continue;
        }

        reporter.report();

        log::info!("App is down :(");

        break;
    }

    Ok(())
}
