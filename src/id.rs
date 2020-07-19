use std::sync::atomic;
use std::fmt;
use std::collections::HashMap;

pub(crate) struct Id(usize);

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Id(s) = self;
        write!(
            f,
            "Id({})",
            s
        )
    }
}

pub(crate) struct Allocator {
    new_id: atomic::AtomicUsize,
}

impl Allocator {
    pub const fn new() -> Self {
        Self { new_id: atomic::AtomicUsize::new(0) }
    }

    pub fn allocate(&self) -> Id {
        Id(self.new_id.fetch_add(1, atomic::Ordering::Relaxed))
    }
}