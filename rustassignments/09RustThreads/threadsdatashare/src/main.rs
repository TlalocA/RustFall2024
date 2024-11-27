fn sending_data_across_threads() {
    extern crate rand; // 0.8.5

    use std::thread;
    // multiproducer, single consumer
    use std::sync::mpsc::channel;

    let (sender,reciever) = channel();

    for i in 0..10 {
        let sender = sender.clone();
        thread::spawn(move || {
            println!("sending: {}",i);
            sender.send(i).unwrap(); // any data could be passed to reciever
            // as well as sending could fail
        });
    }

    for _ in 0..10 {
        let msg = reciever.recv().unwrap();
        println!(" recieved {}", msg );
    }
    // what is important to notice, data will be send and recieved in random order
    // but you will get them in exact order, just be aware of potential queue

    // basically CPU whim

}

fn fortune_cookie() {
    extern crate rand;
    use rand::Rng;
    use std::thread;
    // multiproducer, single consumer
    use std::sync::mpsc::channel;
    
    use std::time;

    let ten_millis = time::Duration::from_millis(1000);
    
    const DISCONNECT: &str = "Come back tomorrow!";
    
    let (sender,reciever) = channel();
    
    
    
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            
            let msg = match rng.gen_range(0..5)  {
                0 => "Fortune favors the brave.",
                1 => DISCONNECT,
                2 => "You will travel to many exotic places in your lifetime.",
                3 => "You can make your own happiness.",
                4 => "You are very talented in many ways.",
                _ => unreachable!(),
            };
            
            println!("Sending cookie: {}",msg);
            //thread::sleep(ten_millis);
            sender.send(msg).unwrap();
            if msg == DISCONNECT {
                break;
            }
        }
    });
    
    for recieved_msg in reciever {
        println!("What a day. Your fortune cookie : {}",recieved_msg);
        thread::sleep(ten_millis);
        
    }
    
}

fn rw_locks() {
    // comparing with mutex which does not separate with reading and writing
    // to the data inside of mutex Read and Write lock, allows to separate that logic
    // like many readers and single writer, very close to RefCell.
    // but this idea is not new or unique to Rust, this idea existed before dawn
    // in Java and C++
    
    use std::sync::{Arc, RwLock};
    use std::thread;
    
    let data = Arc::new(RwLock::new("Hello World".to_string()));
    use std::time;
    let ten_millis = time::Duration::from_millis(10);
    let twenty_millis = time::Duration::from_millis(40);
    
    let reader_a = {
        let data = data.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let data_for_read = data.read().unwrap();
                println!("Data from reader_A: {} ",data_for_read);
                thread::sleep(ten_millis);
            }
        })
    };
    
    let reader_b = {
        let data = data.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let data_for_read = data.read().unwrap();
                println!("Data from reader_B: {} ",data_for_read);
                thread::sleep(ten_millis);
            }
        })
    };
    
    let writer = {
        let data = data.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let mut data_to_write = data.write().unwrap();
                data_to_write.push('!');
                println!("Updating data {} ",data_to_write);
                thread::sleep(twenty_millis);
                
            }
        })
        };
        
    reader_a.join().unwrap();
    reader_b.join().unwrap();
    writer.join().unwrap();
    
    println!("{:?}",data);
    
}

fn arc_example(){

}

fn mutex_example(){
    
}

fn main() {
    arc_example();
    mutex_example();
    sending_data_across_threads();
    fortune_cookie();
    rw_locks();
    
}