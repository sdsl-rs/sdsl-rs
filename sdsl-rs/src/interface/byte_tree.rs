use crate::meta;
use crate::{backend::sdsl_c, interface::common::Ptr};
use anyhow::{format_err, Result};

use crate::interface::common::{self, Code, Id, ParameterValues};

pub struct ByteTree {
    // // Dummy fields which are never used, always None. Included so that generic parameters are used.
    // _bs: Option<Shape>,
    ptr: common::VoidPtr,
    // interface: Interface,
}

impl Code for ByteTree {
    fn c_code() -> Result<String> {
        let meta =
            Box::new(meta::common::bit_patterns::P0Meta::new()) as Box<dyn meta::common::Meta>;
        let parameter_values = Self::parameter_values()?;
        Ok(meta.c_code(&parameter_values)?)
    }
}

impl ParameterValues for ByteTree {
    fn parameter_values() -> Result<Vec<String>> {
        Ok(vec![])
    }
}
