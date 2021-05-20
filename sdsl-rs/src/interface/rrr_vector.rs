use crate::meta;
use crate::{backend::sdsl_c, interface::common::Ptr};
use anyhow::{format_err, Result};

use crate::interface::common::{self, Id, ParameterValues};

/// $ H_0 $-compressed bit vector representation.
///
/// # Arguments
/// * `BlockStore` - A random access integer vector or wavelet tree used to store the block types.
/// * `BLOCK_SIZE` - Size of a basic block.
/// * `RANK_STORE_FREQ` - A rank sample value is stored before every t_k-th basic block.
///
/// # References
/// - Rasmus Pagh
///   Low redundancy in dictionaries with O(1) worst case lookup time
///   Technical Report 1998.
///   ftp://ftp.cs.au.dk/BRICS/Reports/RS/98/28/BRICS-RS-98-28.pdf, Section 2.
/// - Rajeev Raman, V. Raman and S. Srinivasa Rao
///   Succinct Indexable Dictionaries with Applications to representations
///   of k-ary trees and multi-sets.
///   SODA 2002.
/// - Francisco Claude, Gonzalo Navarro:
///   Practical Rank/Select Queries over Arbitrary Sequences.
///   SPIRE 2008: 176-187
/// - On the fly-decoding and encoding was discovered in;
///   Gonzalo Navarro, Eliana Providel:
///   Fast, Small, Simple Rank/Select on Bitmaps.
///   SEA 2012
///
/// # Example
///
/// ```ignore
/// let bv = sdsl::bit_vector! {1, 1, 0, 1};
/// let rv = sdsl::RrrVector::<sdsl::IntVector<0>, 10, 2>::new(&bv)?;
///
/// let result = rv.get_int(1, 3);
/// let expected = 5;
/// assert_eq!(result, expected);
/// ```
///
/// For further examples see [here](https://github.com/sdsl-rs/sdsl-rs/blob/master/examples/src/rrr_vector.rs).

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
    /// Construct a new $ H_0 $-compressed bit vector.
    /// # Arguments
    /// * `bit_vector` - Uncompressed bit vector.
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

    /// Load vector from file.
    /// # Arguments
    /// * `path` - File path.
    pub fn from_file(path: &std::path::PathBuf) -> Result<Self> {
        let rrr_vector = Self::default()?;

        let path = path
            .to_str()
            .ok_or(format_err!("Failed to convert PathBuf into str."))?;
        let path = std::ffi::CString::new(path)?;

        (rrr_vector.interface.io.load_from_file)(rrr_vector.ptr, path.as_ptr());
        Ok(rrr_vector)
    }

    fn default() -> Result<Self> {
        let id = Self::id()?;
        let interface = Interface::new(&id)?;
        let ptr = (interface.default)();

        Ok(Self {
            _bs: None,
            ptr,
            interface,
        })
    }

    /// Returns the length of the original bit vector.
    pub fn len(&self) -> usize {
        (self.interface.len)(self.ptr)
    }

    /// Accessing the i-th element of the original bit vector.
    /// # Arguments
    /// * `index` - An index in range $ [0, \mathrm{len}()) $.
    pub fn get_bv_element(&self, index: usize) -> usize {
        (self.interface.get_bv_element)(self.ptr, index)
    }

    /// Get the integer value of the binary string of length `len` starting at position `index`.
    /// # Arguments
    /// * `index` - Starting index of the binary representation of the integer. $ \mathrm{index} + \mathrm{len} -1 \in [0..\mathrm{size}()-1] $
    /// * `len` - Length of the binary representation of the integer. $ \mathrm{len} \in [1..64] $
    pub fn get_int(&self, index: usize, len: u8) -> usize {
        (self.interface.get_int)(self.ptr, index, len)
    }

    /// Returns an iterator over the original bit vector values.
    pub fn iter_bv(&self) -> common::VectorIterator<Self> {
        common::VectorIterator::new(&self, self.len())
    }

    /// Returns an iterator over integer values of the binary string.
    /// # Arguments
    /// * `len` - Length of the binary representation of the integer. $ \mathrm{len} \in [1..64] $
    pub fn iter_int(&self, len: u8) -> RrrVectorIntIterator<Self> {
        RrrVectorIntIterator::new(&self, self.len(), len)
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
