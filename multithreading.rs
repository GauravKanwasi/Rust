use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for id in 0..5 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50 * id as u64));

            let mut num = counter.lock().unwrap();
            *num += 1;

            println!("[Thread {id}] Hello World!! (count = {num})");

            id * 2
        });

        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        match handle.join() {
            Ok(value) => results.push(value),
            Err(e) => eprintln!("A thread panicked: {e:?}"),
        }
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
    println!("Collected results: {results:?}");
}
