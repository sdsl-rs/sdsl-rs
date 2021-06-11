use crate::backend::sdsl_c;
use anyhow::Result;

use crate::interface::common;

#[derive(Clone)]
pub struct Interface {
    set_to_value: extern "C" fn(common::VoidPtr, u64),
    set_to_id: extern "C" fn(common::VoidPtr),
    set_random_bits: extern "C" fn(common::VoidPtr),
    util_mod: extern "C" fn(common::VoidPtr, u64),
    bit_compress: extern "C" fn(common::VoidPtr),
    expand_width: extern "C" fn(common::VoidPtr, u8),

    _lib: std::sync::Arc<sharedlib::Lib>,
}

impl Interface {
    pub fn new(id: &str) -> Result<Self> {
        let lib = sdsl_c::LIB.clone();
        let builder = sdsl_c::FunctionBuilder::new(None, id, lib.clone());

        Ok(Self {
            set_to_value: builder.get("set_to_value")?,
            set_to_id: builder.get("set_to_id")?,
            set_random_bits: builder.get("set_random_bits")?,
            util_mod: builder.get("mod")?,
            bit_compress: builder.get("bit_compress")?,
            expand_width: builder.get("expand_width")?,

            _lib: lib.clone(),
        })
    }
}

pub trait Util {
    fn util(&self) -> &Interface;
}

pub fn set_to_value<T: Util + common::Ptr>(structure: &mut T, value: u64) {
    (structure.util().set_to_value)(*structure.ptr(), value)
}

pub fn set_to_id<T: Util + common::Ptr>(structure: &mut T) {
    (structure.util().set_to_id)(*structure.ptr())
}

pub fn set_random_bits<T: Util + common::Ptr>(structure: &mut T) {
    (structure.util().set_random_bits)(*structure.ptr())
}

pub fn modulus<T: Util + common::Ptr>(structure: &mut T, value: u64) {
    (structure.util().util_mod)(*structure.ptr(), value)
}

pub fn bit_compress<T: Util + common::Ptr>(structure: &mut T) {
    (structure.util().bit_compress)(*structure.ptr())
}

pub fn expand_width<T: Util + common::Ptr>(structure: &mut T, new_width: u8) {
    (structure.util().expand_width)(*structure.ptr(), new_width)
}

pub mod crate_export {
    pub use super::{
        bit_compress, expand_width, modulus, set_random_bits, set_to_id, set_to_value,
    };
}
