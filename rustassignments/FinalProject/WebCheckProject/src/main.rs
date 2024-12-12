use std::thread;
use std::time::{Duration};
use web_check_project::{handle_threads, read_urls_from_file, WebsiteStatus};

fn main() {
    println!("--- Web Checker Start! ---\n");

    // config
    let file_path = "websites.txt";

    loop {
        let urls = read_urls_from_file(file_path);
        let timeout = Duration::from_secs(3);
        let retries = 3;
    
        let results = handle_threads(urls, timeout, retries);

        println!("--- Status Summary ---");

        
        //iterate through results, return sums of response time
        let average_time: Duration = results.iter().map(|result| result.response_time).sum(); 

        // iterate through results for respective url result
        for result in results.iter() {
            let response = match &result.status {
                Ok(code) => code.to_string(), //OK -> url code response 
                Err(err) => err.clone(), //Err -> url response error
            };
            println!("{}: {}: {} (elapsed time: {:?})", result.timestamp, result.url, response, result.response_time);
        }

        println!("\nAverage response time was: {:?}\n", average_time/results.len() as u32);  // divide time by length of results, type as u32

        println!("--- Checking again in 15 seconds... ---\n");
        thread::sleep(Duration::from_secs(15));
        
    }
}