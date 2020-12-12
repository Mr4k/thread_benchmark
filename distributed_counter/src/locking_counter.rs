use std::sync::{Mutex};

pub struct LockingCounter {
    pub count: Mutex<u64>,
}

impl LockingCounter {
    pub fn increment(&self) {
        let mut mut_count = self.count.lock().unwrap();
        *mut_count += 1;
    }
    pub fn fetch(&self) -> u64 {
        let mut_count = self.count.lock().unwrap();
        return *mut_count
    }
}
