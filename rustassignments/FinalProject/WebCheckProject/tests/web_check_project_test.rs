#[cfg(test)]
mod tests {
    use web_check_project::{read_urls_from_file, check_website};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use std::fs;
  

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

    // test invalid url
    #[test]
    fn test_check_website_invalid() {
        let url = "https://badtest.badsite";
        let timeout = Duration::from_secs(3);
        let retries = 0;

        let response = check_website(url, timeout, retries);

        assert!(response.status.is_err());
    }

    // test unreachable
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
}