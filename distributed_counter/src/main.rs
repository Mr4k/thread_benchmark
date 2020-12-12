use std::{sync::Arc, sync::{RwLock}, thread};
use std::env;

mod distributed_counter;

use distributed_counter::LockingCounter;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_threads: usize = args[1].parse().unwrap();
    let num_writes: usize = args[2].parse().unwrap();
    
    let counter = Arc::new(LockingCounter{counts: (0..(num_threads * 20)).map(|_| { RwLock::new(0) }).collect()});
    
    let handles = (0..num_threads).map(|thread_id| {
        let counter  = Arc::clone(&counter);
        thread::spawn(move || {
            for _ in 0..num_writes {
                counter.increment(thread_id);
            }
        })
    });

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(counter.fetch(), (num_threads as u64) * 100000)
}
