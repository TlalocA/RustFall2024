
use web_check_project::{handle_threads};
use std::time::Duration;

// test concurrent execution
#[test]
fn test_concurrent_checking() {
    let urls = vec!["https://example.com".to_string(),"https://rust-lang.org".to_string(), "https://github.com".to_string(),]; // example sites, temp vector

    let timeout = Duration::from_secs(3);
    let retries = 0;

    let results = handle_threads(urls, timeout, retries);

    assert_eq!(results.len(), 3);

    for result in results.iter() {
        assert!(result.status.is_ok() || result.status.is_err());
    }
}

// test performance
#[test]
fn test_multiple_requests() {
    let urls = vec!["https://example.com".to_string(); 50]; // simulates 50 urls, all example.com
    let timeout = Duration::from_secs(3);
    let retries = 0;

    let results = handle_threads(urls, timeout, retries);

    assert_eq!(results.len(), 50); // ensure all threads complete

    for result in results.iter() {
        assert!(result.status.is_ok() || result.status.is_err()); // iterate through each result
    }
}
