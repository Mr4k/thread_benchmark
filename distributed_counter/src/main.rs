use std::{sync::Arc, sync::{Mutex}, thread};
use std::env;

mod locking_counter;

use locking_counter::LockingCounter;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_threads: usize = args[1].parse().unwrap();
    let num_writes: usize = args[2].parse().unwrap();
    
    let counters: Vec<Arc<LockingCounter>> = (0..num_threads).map(|_| { Arc::new(LockingCounter{count: Mutex::new(0)}) }).collect();
    
    let handles = (0..num_threads).map(|thread_id| {
        let counter = Arc::clone(&counters[thread_id]);
        thread::spawn(move || {
            for _ in 0..num_writes {
                counter.increment();
            }
            counter.fetch()
        })
    });

    let mut sum = 0;
    for h in handles {
        sum += h.join().unwrap();
    }

    assert_eq!(sum, (num_threads * num_writes) as u64)
}
