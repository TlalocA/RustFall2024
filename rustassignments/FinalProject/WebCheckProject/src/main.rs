use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration};
use web_check_project::{read_urls_from_file, check_website, WebsiteStatus};

fn main() {
    println!("--- Web Checker Start! ---");

    // config
    let file_path = "websites.txt"; // Specify the file containing website URLs

    loop {
        let urls = read_urls_from_file(file_path);
        let timeout = Duration::from_secs(3);
        let retries = 3;

    // thread, amount of threads is determined by amount of lines read
    // (possibly very inefficent and unoptimized, but working solution)
    
        let results = Arc::new(Mutex::new(Vec::new()));

        for (step, url) in urls.into_iter().enumerate() {
            let results = Arc::clone(&results);
            println!("--- Attempting connection to {}... ---", url);

            let thread = thread::spawn(move || {
                let website_status = check_website(&url, timeout, retries);

                let mut results = results.lock().unwrap(); // mutex locking for thread sync
                results.push(website_status); // push to status result vector

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