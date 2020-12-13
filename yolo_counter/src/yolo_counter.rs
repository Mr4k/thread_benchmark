pub struct LockingCounter {
    pub count: u64,
}

impl LockingCounter {
    pub fn increment(&self) {
        unsafe {
            // below is an unholy incantation to get a mut pointer to a non mut reference
            // obviously this is incredibly dangerous (which is the point of this whole example)
            // https://stackoverflow.com/questions/53458784/why-is-casting-a-const-reference-directly-to-a-mutable-reference-invalid-in-rust
            let s = (&self.count) as *const u64 as *mut u64;
            *s += 1;
        }
    }
    pub fn fetch(&self) -> u64 {
        return self.count;
    }
}
