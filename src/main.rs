mod reporter;

use std::env;
use dotenv::dotenv;
use log;
use chrono;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    setup_logger()?;
    
    let mut reporter = Reporter::new(env::var("REPORTER_ENDPOINT").expect("REPORTER_ENDPOINT env not set!"));

    let email_reporter = ReporterKind::EmailReporter(EmailReporter::new(None, None, None, None, None, None));
    reporter.add_reporter(email_reporter);

    reporter.report();

    Ok(())
}
