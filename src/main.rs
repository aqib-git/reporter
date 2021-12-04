use std::{thread, time::Duration};

struct Reporter {
    endpoint: String,
}

impl Reporter {
    fn check_status(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let status = reqwest::blocking::get(&self.endpoint)?
            .status();

        return Ok(status == 200)
    }

    fn report(&self) {
        loop {
            thread::sleep(Duration::from_secs(1));

            if self.check_status().unwrap() {
                println!("App is working :)");
        
                continue;
            }
        
            println!("App is down :(");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reporter = Reporter{
        endpoint: "https://app.leadgenapp.io".to_string()
    };

    reporter.report();

    Ok(())
}
