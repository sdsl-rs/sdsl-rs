use crate::meta;
use crate::{backend::sdsl_c, interface::common::Ptr};
use anyhow::{format_err, Result};

use crate::interface::common::{self, Id, ParameterValues};

pub struct RrrVector<BlockStore, const BLOCK_SIZE: u16, const RANK_STORE_FREQ: u16>
where
    BlockStore: common::Code,
{
    // Dummy field so BlockStore is used, always None.
    _bs: Option<BlockStore>,
    ptr: common::VoidPtr,
    interface: Interface,
}

impl<BlockStore, const BLOCK_SIZE: u16, const RANK_STORE_FREQ: u16>
    RrrVector<BlockStore, BLOCK_SIZE, RANK_STORE_FREQ>
where
    BlockStore: common::Code,
{
    pub fn new(bit_vector: &crate::interface::bit_vector::BitVector) -> Result<Self> {
        let id = Self::id()?;
        let interface = Interface::new(&id)?;
        let ptr = (interface.create)(*bit_vector.ptr());

        Ok(Self {
            _bs: None,
            ptr,
            interface,
        })
    }

    pub fn default() -> Result<Self> {
        let id = Self::id()?;
        let interface = Interface::new(&id)?;
        let ptr = (interface.default)();

        Ok(Self {
            _bs: None,
            ptr,
            interface,
        })
    }

    pub fn len(&self) -> usize {
        (self.interface.len)(self.ptr)
    }

    pub fn get_bv_element(&self, index: usize) -> usize {
        (self.interface.get_bv_element)(self.ptr, index)
    }

    pub fn get_int(&self, index: usize, len: u8) -> usize {
        (self.interface.get_int)(self.ptr, index, len)
    }

    pub fn iter_bv(&self) -> common::VectorIterator<Self> {
        common::VectorIterator::new(&self, self.len())
    }

    pub fn iter_int(&self, int_len: u8) -> RrrVectorIntIterator<Self> {
        RrrVectorIntIterator::new(&self, self.len(), int_len)
    }

    pub fn from_file(path: &std::path::PathBuf) -> Result<Self> {
        let rrr_vector = Self::default()?;

        let path = path
            .to_str()
            .ok_or(format_err!("Failed to convert PathBuf into str."))?;
        let path = std::ffi::CString::new(path)?;

        (rrr_vector.interface.io.load_from_file)(rrr_vector.ptr, path.as_ptr());
        Ok(rrr_vector)
    }
}

impl<BlockStore, const BLOCK_SIZE: u16, const RANK_STORE_FREQ: u16> common::io::IO
    for RrrVector<BlockStore, BLOCK_SIZE, RANK_STORE_FREQ>
where
    BlockStore: common::Code,
{
    fn io(&self) -> &common::io::Interface {
        &self.interface.io
    }
}

impl<BlockStore, const BLOCK_SIZE: u16, const RANK_STORE_FREQ: u16> common::Ptr
    for RrrVector<BlockStore, BLOCK_SIZE, RANK_STORE_FREQ>
where
    BlockStore: common::Code,
{
    fn ptr(&self) -> &common::VoidPtr {
        &self.ptr
    }
}

impl<BlockStore, const BLOCK_SIZE: u16, const RANK_STORE_FREQ: u16> common::Id
    for RrrVector<BlockStore, BLOCK_SIZE, RANK_STORE_FREQ>
where
    BlockStore: common::Code,
{
    fn id() -> Result<String> {
        let meta = Box::new(meta::rrr_vector::RrrVectorMeta::new()) as Box<dyn meta::common::Meta>;
        let parameter_values = Self::parameter_values()?;
        let id = sdsl_c::specification::get_id(&parameter_values, &meta)?;
        Ok(id)
    }
}

impl<BlockStore, const BLOCK_SIZE: u16, const RANK_STORE_FREQ: u16> common::Code
    for RrrVector<BlockStore, BLOCK_SIZE, RANK_STORE_FREQ>
where
    BlockStore: common::Code,
{
    fn c_code() -> Result<String> {
        let meta = Box::new(meta::rrr_vector::RrrVectorMeta::new()) as Box<dyn meta::common::Meta>;
        let parameter_values = Self::parameter_values()?;
        Ok(meta.c_code(&parameter_values)?)
    }
}

impl<BlockStore, const BLOCK_SIZE: u16, const RANK_STORE_FREQ: u16> common::ParameterValues
    for RrrVector<BlockStore, BLOCK_SIZE, RANK_STORE_FREQ>
where
    BlockStore: common::Code,
{
    fn parameter_values() -> Result<Vec<String>> {
        Ok(vec![
            BlockStore::c_code()?,
            BLOCK_SIZE.to_string(),
            RANK_STORE_FREQ.to_string(),
        ])
    }
}

impl<BlockStore, const BLOCK_SIZE: u16, const RANK_STORE_FREQ: u16> common::IterGet
    for RrrVector<BlockStore, BLOCK_SIZE, RANK_STORE_FREQ>
where
    BlockStore: common::Code,
{
    fn iter_get(&self, index: usize) -> usize {
        (self.interface.get_bv_element)(self.ptr, index)
    }
}

impl<BlockStore, const BLOCK_SIZE: u16, const RANK_STORE_FREQ: u16> Drop
    for RrrVector<BlockStore, BLOCK_SIZE, RANK_STORE_FREQ>
where
    BlockStore: common::Code,
{
    fn drop(&mut self) {
        (self.interface.drop)(self.ptr)
    }
}

impl<BlockStore, const BLOCK_SIZE: u16, const RANK_STORE_FREQ: u16> Clone
    for RrrVector<BlockStore, BLOCK_SIZE, RANK_STORE_FREQ>
where
    BlockStore: common::Code,
{
    fn clone(&self) -> Self {
        Self {
            _bs: None,
            ptr: (self.interface.clone)(self.ptr),
            interface: self.interface.clone(),
        }
    }
}

#[derive(Clone)]
struct Interface {
    create: extern "C" fn(common::VoidPtr) -> common::VoidPtr,
    default: extern "C" fn() -> common::VoidPtr,
    drop: extern "C" fn(common::VoidPtr),
    clone: extern "C" fn(common::VoidPtr) -> common::VoidPtr,
    len: extern "C" fn(common::VoidPtr) -> usize,
    get_bv_element: extern "C" fn(common::VoidPtr, usize) -> usize,
    get_int: extern "C" fn(common::VoidPtr, usize, u8) -> usize,

    pub io: common::io::Interface,
    _lib: std::sync::Arc<sharedlib::Lib>,
}

impl Interface {
    pub fn new(id: &str) -> Result<Self> {
        let lib = sdsl_c::LIB.clone();
        let builder = sdsl_c::FunctionBuilder::new(Some("rrr_vector"), id, lib.clone());

        Ok(Self {
            create: builder.get("create")?,
            default: builder.get("default")?,
            drop: builder.get("destroy")?,
            clone: builder.get("copy")?,
            len: builder.get("size")?,
            get_bv_element: builder.get("get_bv_element")?,
            get_int: builder.get("get_int")?,

            io: common::io::Interface::new(&id)?,
            _lib: lib.clone(),
        })
    }
}

pub trait IterGetInt {
    fn iter_get_int(&self, index: usize, int_len: u8) -> usize;
}

impl<BlockStore, const BLOCK_SIZE: u16, const RANK_STORE_FREQ: u16> IterGetInt
    for RrrVector<BlockStore, BLOCK_SIZE, RANK_STORE_FREQ>
where
    BlockStore: common::Code,
{
    fn iter_get_int(&self, index: usize, int_len: u8) -> usize {
        (self.interface.get_int)(self.ptr, index, int_len)
    }
}

pub struct RrrVectorIntIterator<'a, T: IterGetInt> {
    vector: &'a T,
    len: usize,
    int_len: u8,
    index: usize,
}

impl<'a, T: IterGetInt> RrrVectorIntIterator<'a, T> {
    pub fn new(vector: &'a T, len: usize, int_len: u8) -> Self {
        Self {
            vector,
            len,
            int_len,
            index: 0,
        }
    }
}

impl<'a, T: IterGetInt> Iterator for RrrVectorIntIterator<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let result = if self.index < self.len {
            Some(self.vector.iter_get_int(self.index, self.int_len))
        } else {
            None
        };
        self.index += 1;
        result
    }
}
