pub trait TreeStrategy {
    type Value: Clone + std::fmt::Debug;
}

/// Layout of the tree structure in memory.
pub trait MemoryLayout {
    fn flag() -> bool;
}

pub struct BreadthFirstSearch;

impl MemoryLayout for BreadthFirstSearch {
    fn flag() -> bool {
        false
    }
}

pub struct DepthFirstSearch;

impl MemoryLayout for DepthFirstSearch {
    fn flag() -> bool {
        true
    }
}
