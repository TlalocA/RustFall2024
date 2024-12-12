use std::time::{Duration, Instant};
use chrono::{DateTime, Utc};
use std::fs;
use ureq;
use std::sync::{Arc, Mutex};
use std::thread;

// given website status struct
#[derive(Debug, Clone)] // clone allows for handle thread function to work
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

pub fn handle_threads(urls: Vec<String>, timeout: Duration, retries: u8) -> Vec<WebsiteStatus> {
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut threads = Vec::new();  //stores and handles threads

    for (step, url) in urls.into_iter().enumerate() {
        let results = Arc::clone(&results);

        let thread = thread::spawn(move || {
            let status = check_website(&url, timeout, retries);
            let mut results = results.lock().unwrap(); // mutex locking for thread sync
            results.push(status); // push to status result vector
        
            println!("Thread {} finished checking URL: {}", step + 1, url);
            println!();
        });

        threads.push(thread); // completed thread, store in threads vector
    }

    for thread in threads { // ensures all threads have caught up
        thread.join().unwrap(); 
    }

    let results = results.lock().unwrap().clone(); // clone results for return

    results
}

pub fn read_urls_from_file(file_path: &str) -> Vec<String> {
    let content = fs::read_to_string(file_path).expect("Failed to read file!");
    //map each line, trim whitespace, convert to string, collect result
    content.lines().map(|line| line.trim().to_string()).collect()
}

fn check_website(url: &str, timeout: Duration, retries: u8) -> WebsiteStatus {
    let start = Instant::now(); // start timer
    let mut status = Err("Unknown error".to_string());
    
    for i in 0..=retries {
        // handles connect and read timeouts
        let agent = ureq::AgentBuilder::new().timeout_connect(timeout).timeout_read(timeout).build();

        let result = agent.get(url).call(); // agent fetches url, working solution
        
        match result {
            Ok(response) => {
                println!("({}): {} connection success!", i, url);
                status = Ok(response.status()); // Ok -> valid status
                break; // break on success
            }
            Err(e) => {
                println!("({}): {} connection failed, retrying...", i, url);
                status = Err(e.to_string()); // Err -> error status
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

#[cfg(test)]
mod tests {
    use super::*;

    // test file read
    #[test]
    fn test_read_file() {
        let test_file_path = "test_websites.txt"; // temp test website file
        fs::write(test_file_path, "https://example.com\nhttps://rust-lang.org\n").unwrap();

        let urls = read_urls_from_file(test_file_path);
        assert_eq!(urls.len(), 2);
        assert_eq!(urls[0], "https://example.com");
        assert_eq!(urls[1], "https://rust-lang.org");

        fs::remove_file(test_file_path).unwrap(); // remove file
    }

    // test valid url
    #[test]
    fn test_check_website_valid() {
        let url = "https://example.com";
        let timeout = Duration::from_secs(3);
        let retries = 0;

        let response = check_website(url, timeout, retries);

        match response.status {
            Ok(code) => assert!(code >= 200 && code < 500), // ensure response code is valid 
            Err(_) => panic!("Expected a successful HTTP response"),
        }
    }

    // test invalid url (error handle)
    #[test]
    fn test_check_website_invalid() {
        let url = "https://badtest.badsite";
        let timeout = Duration::from_secs(3);
        let retries = 0;

        let response = check_website(url, timeout, retries);

        assert!(response.status.is_err());
    }

    // test unreachable (error handle)
    #[test]
    fn test_check_website_unreachable() {
        let url = "https://0.0.0.0";
        let timeout = Duration::from_secs(3);
        let retries = 1;
    
        let response = check_website(url, timeout, retries);
    
        assert!(response.status.is_err());
        let error_message = response.status.unwrap_err();
    
        // check if error contains key words
        assert!(
            error_message.contains("Dns Failed") || error_message.contains("resolve dns name") || error_message.contains("Connection refused"),
            "Unexpected error message: {}",
            error_message
        );
    }
}