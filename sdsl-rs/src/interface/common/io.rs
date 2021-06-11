use crate::backend::sdsl_c;
use anyhow::{format_err, Result};

use crate::interface::common;

#[derive(Clone)]
pub struct Interface {
    store_to_file: extern "C" fn(common::VoidPtr, *const std::os::raw::c_char) -> bool,
    store_int_vector_to_file:
        extern "C" fn(common::VoidPtr, *const std::os::raw::c_char, bool) -> bool,
    pub load_from_file: extern "C" fn(common::VoidPtr, *const std::os::raw::c_char) -> bool,

    _lib: std::sync::Arc<sharedlib::Lib>,
}

impl Interface {
    pub fn new(id: &str) -> Result<Self> {
        let lib = sdsl_c::LIB.clone();
        let builder = sdsl_c::FunctionBuilder::new(None, id, lib.clone());

        Ok(Self {
            store_to_file: builder.get("store_to_file")?,
            store_int_vector_to_file: builder.get("store_int_vector_to_file")?,
            load_from_file: builder.get("load_from_file")?,

            _lib: lib.clone(),
        })
    }
}

pub trait IO {
    fn io(&self) -> &Interface;
}

/// Write structure to file.
pub fn store_to_file<T: IO + common::Ptr>(
    structure: &T,
    path: &std::path::PathBuf,
) -> Result<bool> {
    let path = path
        .to_str()
        .ok_or(format_err!("Failed to convert PathBuf into str"))?;
    let path = std::ffi::CString::new(path)?;
    Ok((structure.io().store_to_file)(
        *structure.ptr(),
        path.as_ptr(),
    ))
}

/// Write int vector to file.
pub fn store_int_vector_to_file<T: IO + common::Ptr>(
    structure: &T,
    path: &std::path::PathBuf,
    write_fixed_as_variable: bool,
) -> Result<bool> {
    let path = path
        .to_str()
        .ok_or(format_err!("Failed to convert PathBuf into str"))?;
    let path = std::ffi::CString::new(path)?;
    Ok((structure.io().store_int_vector_to_file)(
        *structure.ptr(),
        path.as_ptr(),
        write_fixed_as_variable,
    ))
}

/// Load structure from file.
pub fn load_from_file<T: IO + common::Ptr>(
    structure: &mut T,
    path: &std::path::PathBuf,
) -> Result<bool> {
    let path = path
        .to_str()
        .ok_or(format_err!("Failed to convert PathBuf into str."))?;
    let path = std::ffi::CString::new(path)?;

    Ok((structure.io().load_from_file)(
        *structure.ptr(),
        path.as_ptr(),
    ))
}

pub mod crate_export {
    pub use super::{load_from_file, store_int_vector_to_file, store_to_file};
}
