use crate::interface::common;
use anyhow::Result;

pub trait TreeStrategy {
    type Value: Clone + std::fmt::Debug;
    const LEX_ORDERED: bool;
}

/// Layout of the tree structure in memory.
pub trait MemoryLayout: common::Code {}

pub struct BreadthFirstSearch;

impl MemoryLayout for BreadthFirstSearch {}

impl common::Code for BreadthFirstSearch {
    fn c_code() -> Result<String> {
        Ok("false".to_string())
    }

    fn parameters_c_code() -> Result<Vec<String>> {
        Ok(vec![])
    }
}

pub struct DepthFirstSearch;

impl MemoryLayout for DepthFirstSearch {}

impl common::Code for DepthFirstSearch {
    fn c_code() -> Result<String> {
        Ok("true".to_string())
    }

    fn parameters_c_code() -> Result<Vec<String>> {
        Ok(vec![])
    }
}
