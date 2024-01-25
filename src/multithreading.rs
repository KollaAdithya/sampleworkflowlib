use std::sync::{Arc, Mutex};
use std::thread;

pub fn start_threads(initial_value: i32) -> i32 {
    let data = Arc::new(Mutex::new(initial_value));
    let threads: Vec<_> = (0..10).map(|_| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
        })
    }).collect();

    for thread in threads {
        thread.join().unwrap();
    }

    let final_result = *data.lock().unwrap();
    final_result // Return the final result
}