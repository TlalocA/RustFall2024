use std::fs; // file system
use std::sync::{Arc, Mutex}; // thread mutex
use std::thread; // thread
use std::time::{Duration, Instant}; // timing library
use chrono::{Utc, DateTime}; // to record timestamps

#[derive(Debug)]
// given website status struct
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: DateTime<Utc>,
}

fn check_website(url: &str, timeout: Duration, retries: u8) -> WebsiteStatus {
    let start = Instant::now(); // start timer
    let mut status = Err("Unknown error".to_string());

    for i in 0..=retries {
        // agent handles timeouts
        let agent = ureq::AgentBuilder::new().timeout_connect(timeout).timeout_read(timeout).build(); // handles connect and read timeouts
        let result = agent.get(url).call(); // working solution

        match result {
            Ok(response) => {
                println!("({}): Fetch success!", i);
                status = Ok(response.status());
                //break; // break match if success
            }
            Err(e) => {
                println!("({}): Fetch failed, retying connection...", i);
                status = Err(e.to_string());
            }
        }
    }

    let response_time = start.elapsed(); // stop timer, get elapsed time
    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time,
        timestamp: Utc::now(),
    }
}

fn read_urls_from_file(file_path: &str) -> Vec<String> {
    let content = fs::read_to_string(file_path).expect("Failed to read file!");
    //map each line, trim whitespace, convert to string, collect result
    content.lines().map(|line| line.trim().to_string()).collect() 
}

fn main() {
    println!("--- Web Checker Start! ---");

    // config
    let file_path = "websites.txt"; // Specify the file containing website URLs

    loop {
        let urls = read_urls_from_file(file_path);
        let timeout = Duration::from_secs(3);
        let retries = 3;

    // thread
    
        let results = Arc::new(Mutex::new(Vec::new()));

        for (step, url) in urls.into_iter().enumerate() {
            let results = Arc::clone(&results);
            println!("--- Attempting connection to {}... ---", url);

            let thread = thread::spawn(move || {
                let website_status = check_website(&url, timeout, retries);

                {
                    let mut results = results.lock().unwrap(); // mutex locking for thread sync
                    results.push(website_status); // push to status result vector
                }

                println!("Thread {} finished checking URL: {}", step + 1, url);
                println!();
            });

            thread.join().unwrap();
        }

        println!("--- Status Summary ---");

        let results = results.lock().unwrap();
        //iterate through results, return sums of response time
        let average_time: Duration = results.iter().map(|result| result.response_time).sum(); 

        // iterate through results for respective url result
        for result in results.iter() {
            let response = match &result.status {
                Ok(code) => code.to_string(), //OK -> URL response 
                Err(err) => err.clone(), //Err -> URL response error
            };
            println!("{}: {}: {} (elapsed time: {:?})", result.timestamp, result.url, response, result.response_time);
        }

        println!("\n--- Average response time was: {:?} ---\n", average_time/results.len() as u32);  // divide time by length of results, type as u32

        println!("--- Checking again in 15 seconds... ---\n");
        thread::sleep(Duration::from_secs(15));
        
    }
}
