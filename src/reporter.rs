mod emailreporter;

pub use crate::reporter::emailreporter::EmailReporter;

pub enum ReporterKind {
    EmailReporter(EmailReporter)
}

pub struct Reporter {
    reporters: Vec<ReporterKind>
}

impl Reporter {
    pub fn new() -> Reporter {
        Reporter {
            reporters: Vec::new()
        }
    }

    pub fn add_reporter(&mut self, reporter: ReporterKind) {
        self.reporters.push(reporter);
    }

    pub fn report(&mut self) {
        for reporter in &mut self.reporters {
            match reporter {
                ReporterKind::EmailReporter(reporter) => reporter.report(),
            }
        }
    }
}
