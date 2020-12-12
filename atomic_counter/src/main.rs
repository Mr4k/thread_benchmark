use std::{sync::Arc, sync::atomic::AtomicU64, thread};
use std::env;

mod atomic_counter;

use atomic_counter::LockingCounter;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_threads: usize = args[1].parse().unwrap();
    let num_writes: usize = args[2].parse().unwrap();
    
    let counter = Arc::new(LockingCounter{count: AtomicU64::new(0)});
    
    let handles = (0..num_threads).map(|_| {
        let counter  = Arc::clone(&counter);
        thread::spawn(move || {
            for _ in 0..num_writes {
                counter.increment();
            }
        })
    });

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(counter.fetch(), (num_threads * num_writes) as u64)
}
