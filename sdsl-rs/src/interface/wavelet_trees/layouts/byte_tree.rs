use crate::meta;
use anyhow::Result;

use super::common;
use crate::interface::common::Code;

pub struct ByteTree<Layout: common::MemoryLayout = common::BreadthFirstSearch> {
    _x: Option<Layout>,
}

impl<Layout: common::MemoryLayout> Code for ByteTree<Layout> {
    fn c_code() -> Result<String> {
        let meta = Box::new(meta::wavelet_trees::layouts::byte_tree::ByteTreeMeta::new())
            as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        Ok(meta.c_code(&parameters_c_code)?)
    }

    fn parameters_c_code() -> Result<Vec<String>> {
        Ok(vec![Layout::c_code()?])
    }
}

impl<Layout: common::MemoryLayout> common::TreeStrategy for ByteTree<Layout> {
    type Value = u8;
    type Size = usize;
    const LEX_ORDERED: bool = false;
}
