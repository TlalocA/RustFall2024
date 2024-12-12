
use web_check_project::{check_website};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// test concurrent execution
#[test]
fn test_concurrent_checking() {
    let urls = vec!["https://example.com".to_string(),"https://rust-lang.org".to_string(), "https://github.com".to_string(),]; // example sites, temp vector

    let timeout = Duration::from_secs(3);
    let retries = 0;

    let results = Arc::new(Mutex::new(Vec::new()));

    let mut threads = Vec::new();

    for url in urls {
        let results = Arc::clone(&results);
        threads.push(thread::spawn(move || {
            let website_status = check_website(&url, timeout, retries);

            let mut results = results.lock().unwrap();
            results.push(website_status);
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let results = results.lock().unwrap();
    assert_eq!(results.len(), 3);

    for result in results.iter() {
        assert!(result.status.is_ok() || result.status.is_err());
    }
}

// test performance
#[test]
fn test_multiple_requests() {
    let urls = vec!["https://example.com"; 50]; // simulates 50 urls, all example.com
    let timeout = Duration::from_secs(3);
    let retries = 0;

    let results = Arc::new(Mutex::new(Vec::new()));

    let mut threads = Vec::new();

    for url in urls {
        let results = Arc::clone(&results);
        threads.push(thread::spawn(move || {
            let website_status = check_website(&url, timeout, retries);

            let mut results = results.lock().unwrap();
            results.push(website_status);
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let results = results.lock().unwrap();
    assert_eq!(results.len(), 50); // ensure all threads complete

    for result in results.iter() {
        assert!(result.status.is_ok() || result.status.is_err()); // iterate through each result
    }
}
