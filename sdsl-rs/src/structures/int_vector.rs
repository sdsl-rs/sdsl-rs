use crate::backend::sdsl_c;
use crate::meta;
use anyhow::{format_err, Result};

use crate::structures::common;

pub struct IntVector<const WIDTH: u8> {
    ptr: common::VoidPtr,
    interface: Interface,
}

impl<const WIDTH: u8> IntVector<WIDTH> {
    pub fn new(size: u64, default_value: u64, width: Option<u8>) -> Result<Self> {
        let meta = Box::new(meta::int_vector::IntVectorMeta::new()) as Box<dyn meta::common::Meta>;
        let id = sdsl_c::specification::get_id(&Some(&vec![WIDTH.to_string()]), &meta)?;

        assert!(
            (WIDTH == 0 && width.is_some()) || (WIDTH != 0 && width.is_none()),
            "Width argument must be specified iff WIDTH const generic value is 0."
        );
        let width = match width {
            Some(width) => width,
            None => WIDTH,
        };

        let interface = Interface::new(&id)?;
        let ptr = (interface.create)(size, default_value, width);

        Ok(Self { ptr, interface })
    }

    pub fn len(&self) -> usize {
        (self.interface.len)(self.ptr)
    }

    pub fn get(&self, pos: u64) -> usize {
        (self.interface.get)(self.ptr, pos)
    }

    pub fn set(&mut self, pos: u64, value: u64) {
        (self.interface.set)(self.ptr, pos, value)
    }

    pub fn is_empty(&self) -> bool {
        (self.interface.is_empty)(self.ptr)
    }

    pub fn resize(&mut self, size: usize) {
        (self.interface.resize)(self.ptr, size)
    }

    pub fn bit_resize(&mut self, size: usize) {
        (self.interface.bit_resize)(self.ptr, size)
    }

    pub fn bit_size(&self) -> usize {
        (self.interface.bit_size)(self.ptr)
    }

    pub fn capacity(&self) -> usize {
        (self.interface.capacity)(self.ptr)
    }

    pub fn data(&self) -> common::VoidPtr {
        // TODO: Tie pointer lifetime to self.
        (self.interface.data)(self.ptr)
    }

    pub fn width(&self) -> u8 {
        (self.interface.width)(self.ptr)
    }

    pub fn set_width(&mut self, value: usize) -> Result<()> {
        if WIDTH != 0 {
            Err(format_err!(
                "WIDTH is non-zero. Width is therefore immutable."
            ))
        } else {
            Ok((self.interface.set_width)(self.ptr, value))
        }
    }

    pub fn store_to_file(
        &self,
        path: &std::path::PathBuf,
        write_fixed_as_variable: bool,
    ) -> Result<bool> {
        let path = path
            .to_str()
            .ok_or(format_err!("Failed to convert PathBuf into str"))?;
        let path = std::ffi::CString::new(path)?;
        Ok((self.interface.store_to_file)(
            self.ptr,
            path.as_ptr(),
            write_fixed_as_variable,
        ))
    }

    pub fn from_file(size: u64, width: u8, path: &std::path::PathBuf) -> Result<Self> {
        assert!(
            WIDTH == 0,
            "Generic const WIDTH must be zero when loading from file."
        );
        let int_vector = Self::new(size, 0, Some(width))?;

        let path = path
            .to_str()
            .ok_or(format_err!("Failed to convert PathBuf into str."))?;
        let path = std::ffi::CString::new(path)?;

        (int_vector.interface.load_from_file)(int_vector.ptr, path.as_ptr());
        Ok(int_vector)
    }
}

impl<const WIDTH: u8> Drop for IntVector<WIDTH> {
    fn drop(&mut self) {
        (self.interface.drop)(self.ptr)
    }
}

impl<const WIDTH: u8> Clone for IntVector<WIDTH> {
    fn clone(&self) -> Self {
        Self {
            ptr: (self.interface.clone)(self.ptr),
            interface: self.interface.clone(),
        }
    }
}

#[derive(Clone)]
struct Interface {
    create: extern "C" fn(u64, u64, u8) -> common::VoidPtr,
    drop: extern "C" fn(*mut libc::c_void),
    clone: extern "C" fn(*mut libc::c_void) -> common::VoidPtr,
    len: extern "C" fn(*mut libc::c_void) -> usize,
    get: extern "C" fn(*mut libc::c_void, u64) -> usize,
    set: extern "C" fn(*mut libc::c_void, u64, u64),
    is_empty: extern "C" fn(*mut libc::c_void) -> bool,
    resize: extern "C" fn(*mut libc::c_void, usize),
    bit_resize: extern "C" fn(*mut libc::c_void, usize),
    bit_size: extern "C" fn(*mut libc::c_void) -> usize,
    capacity: extern "C" fn(*mut libc::c_void) -> usize,
    data: extern "C" fn(*mut libc::c_void) -> common::VoidPtr,
    width: extern "C" fn(*mut libc::c_void) -> u8,
    set_width: extern "C" fn(*mut libc::c_void, usize),

    store_to_file: extern "C" fn(*mut libc::c_void, *const std::os::raw::c_char, bool) -> bool,
    load_from_file: extern "C" fn(*mut libc::c_void, *const std::os::raw::c_char) -> bool,

    _lib: std::sync::Arc<sharedlib::Lib>,
}

impl Interface {
    pub fn new(id: &str) -> Result<Self> {
        let lib = sdsl_c::LIB.clone();
        let builder = sdsl_c::FunctionBuilder::new("int_vector", id, lib.clone());

        Ok(Self {
            create: builder.get("create")?,
            drop: builder.get("destroy")?,
            clone: builder.get("copy")?,
            len: builder.get("size")?,
            get: builder.get("get_element")?,
            set: builder.get("set_element")?,
            is_empty: builder.get("empty")?,
            resize: builder.get("resize")?,
            bit_resize: builder.get("bit_resize")?,
            bit_size: builder.get("bit_size")?,
            capacity: builder.get("capacity")?,
            data: builder.get("data")?,
            width: builder.get("width")?,
            set_width: builder.get("set_width")?,

            store_to_file: builder.get("store_to_file")?,
            load_from_file: builder.get("load_from_file")?,

            _lib: lib.clone(),
        })
    }
}
