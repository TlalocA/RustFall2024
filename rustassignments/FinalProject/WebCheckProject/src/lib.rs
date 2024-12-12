use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use std::fs;
use ureq;

// naming this lib.rs proved to be a working solution
// not too sure what was meant by "modularize", thought separating functions from main would be ideal

// given website status struct
#[derive(Debug)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

pub fn read_urls_from_file(file_path: &str) -> Vec<String> {
    let content = fs::read_to_string(file_path).expect("Failed to read file!");
    content.lines().map(|line| line.trim().to_string()).collect()
}

pub fn check_website(url: &str, timeout: Duration, retries: u8) -> WebsiteStatus {
    let start = Instant::now(); // start timer
    let mut status = Err("Unknown error".to_string());

    for i in 0..=retries {
        // handles connect and read timeouts
        let agent = ureq::AgentBuilder::new().timeout_connect(timeout).timeout_read(timeout).build();

        let result = agent.get(url).call(); // working solution

        match result {
            Ok(response) => {
                println!("({}): Fetch success!", i);
                status = Ok(response.status());
                //break; // break match if success
            }
            Err(e) => {
                println!("({}): Fetch failed, retrying...", i);
                status = Err(e.to_string());
            }
        }
    }

    let response_time = start.elapsed(); // stop timer, get elapsed time
    // return struct
    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time,
        timestamp: Utc::now(),
    }
}