use anyhow::Result;
pub mod byte_tree;
pub mod common;

pub fn get_metas() -> Result<Vec<Box<dyn crate::meta::common::Meta>>> {
    Ok(vec![
        Box::new(common::BreadthFirstSearchMeta::new()) as Box<dyn crate::meta::common::Meta>,
        Box::new(byte_tree::ByteTreeMeta::new()) as Box<dyn crate::meta::common::Meta>,
    ])
}
