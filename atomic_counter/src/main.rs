use std::{sync::Arc, sync::atomic::AtomicU64, thread, thread::JoinHandle};
use std::env;

mod atomic_counter;

use atomic_counter::LockingCounter;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_threads: usize = args[1].parse().unwrap();
    let num_total_writes: usize = args[2].parse().unwrap();
    let num_writes_per_thread = num_total_writes / num_threads;
    let remainder_writes: usize = num_total_writes % num_threads;
    
    let counter = Arc::new(LockingCounter{count: AtomicU64::new(0)});
    
    let handles: Vec<JoinHandle<_>> = (0..num_threads).map(|thread_id| {
        let counter  = Arc::clone(&counter);
        let num_writes = if thread_id == 0 { num_writes_per_thread + remainder_writes } else { num_writes_per_thread };
        thread::spawn(move || {
            for _ in 0..num_writes {
                counter.increment();
            }
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    assert_eq!(counter.fetch(), num_total_writes as u64)
}
