use std::fmt;
use std::sync::atomic;

#[derive(Clone, Copy, std::cmp::Eq, std::hash::Hash)]
pub struct NodeId(usize);

impl std::cmp::PartialEq for NodeId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let NodeId(s) = self;
        write!(f, "Id({})", s)
    }
}

pub(crate) struct Allocator {
    new_id: atomic::AtomicUsize,
}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            new_id: atomic::AtomicUsize::new(0),
        }
    }

    pub fn allocate(&self) -> NodeId {
        NodeId(self.new_id.fetch_add(1, atomic::Ordering::Relaxed))
    }
}
