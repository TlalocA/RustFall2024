use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let shared_val = Arc::new(Mutex::new(0));

    let steps =  Box::new(5);

    for step in 1..=*steps {
        let shared_val = Arc::clone(&shared_val); 

        let thread = thread::spawn(move || {
            for _ in 0..10 {
                let mut count = shared_val.lock().unwrap();
                *count += 1;
                //println!("Thread {} incremented value to {}", step, *count); // debug
            }

            println!("Thread {} finished counting", step);
        });

        thread.join().unwrap();
    }

    let result = *shared_val.lock().unwrap();
    println!("Final Count: {}", result);
}