use std::sync::atomic::{AtomicU64, Ordering};

pub struct LockingCounter {
    pub count: AtomicU64,
}

impl LockingCounter {
    pub fn increment(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }
    pub fn fetch(&self) -> u64 {
        return self.count.load(Ordering::Relaxed);
    }
}
