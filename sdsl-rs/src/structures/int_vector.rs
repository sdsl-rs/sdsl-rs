use crate::backend::sdsl_c;
use crate::meta;
use anyhow::Result;

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
            "Width argument must be specified if WIDTH const generic value is 0."
        );
        let width = match width {
            Some(width) => width,
            None => WIDTH,
        };

        let lib = sdsl_c::get_lib()?;
        let interface = Interface::new(&id, lib)?;
        let ptr = (interface.create)(size, default_value, width);

        Ok(Self {
            ptr,
            interface,
        })
    }

    pub fn get(&self, pos: u64) -> usize {
        (self.interface.get)(self.ptr, pos)
    }
}

impl<const WIDTH: u8> Drop for IntVector<WIDTH> {
    fn drop(&mut self) {
        (self.interface.drop)(self.ptr)
    }
}

struct Interface {
    create: extern "C" fn(u64, u64, u8) -> common::VoidPtr,
    get: extern "C" fn(*mut libc::c_void, u64) -> usize,
    drop: extern "C" fn(*mut libc::c_void),
    _lib: sharedlib::Lib,
}

impl Interface {
    pub fn new(id: &String, lib: sharedlib::Lib) -> Result<Self> {
        Ok(Self {
            create: sdsl_c::get_function("int_vector_create", Some(&id), &lib)?,
            get: sdsl_c::get_function("int_vector_get_element", Some(&id), &lib)?,
            drop: sdsl_c::get_function("int_vector_destroy", Some(&id), &lib)?,
            _lib: lib,
        })
    }
}