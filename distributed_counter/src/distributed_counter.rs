use std::sync::{RwLock};

pub struct LockingCounter {
    pub counts: Vec<RwLock<u64>>,
}

impl LockingCounter {
    pub fn increment(&self, thread_id: usize) {
        let lock = &self.counts[thread_id*20];
        let mut val = lock.write().unwrap();
        *val += 1;
    }
    pub fn fetch(&self) -> u64 {
        let mut sum: u64 = 0;
        for thread in &self.counts {
            let count =  thread.read().unwrap();
            sum += count.clone();
        }
        return sum
    }
}
