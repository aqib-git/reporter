mod emailreporter;

use log;
use std::{thread, time::Duration};
pub use crate::reporter::emailreporter::EmailReporter;

pub enum ReporterKind {
    EmailReporter(EmailReporter)
}

pub struct Reporter {
    endpoint: String,
    reporters: Vec<ReporterKind>
}

impl Reporter {
    pub fn new(endpoint: String) -> Reporter {
        Reporter {
            endpoint: String::from(endpoint),
            reporters: Vec::new()
        }
    }

    pub fn add_reporter(&mut self, reporter: ReporterKind) {
        self.reporters.push(reporter);
    }

    fn check_status(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let status = reqwest::blocking::get(&self.endpoint)?
            .status();

        return Ok(status == 200)
    }

    pub fn report(&mut self) {
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
