use std::thread;

fn main() {
    let steps =  Box::new(3);
    let thread = thread::spawn(move ||{
        for step in 1..=*steps {
            println!("Thread {}",step);
        }
        "All threads completed!"
    });
    
    let result = thread.join().unwrap();

    println!("{}", result);
}