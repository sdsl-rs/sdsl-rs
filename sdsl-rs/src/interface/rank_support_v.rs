use crate::backend::sdsl_c;
use crate::meta;
use anyhow::{format_err, Result};

use crate::interface::common::{self, Code, Id, Ptr};

/// A rank structure proposed by Sebastiano Vigna.
///
/// Space complexity $ 0.25n $ for a bit vector of length n bits.
///
/// The superblock size is 512. Each superblock is subdivided into 512/64 = 8
/// blocks. So absolute counts for the superblock add 64/512 bits on top of each
/// supported bit. Since the first of the 8 relative count values is 0, we can
/// fit the remaining 7 (each of width log(512)=9) in a 64bit word. The relative
/// counts add another 64/512 bits on top of each supported bit.
/// In total this results in 128/512=25% overhead.
///
/// # Arguments
/// * `BitPattern` - Bit pattern `0`,`1`,`10`,`01` which should be ranked.
///
/// # Example
/// ```ignore
/// let bv = sdsl::bit_vector! {0, 1, 0, 1, 0, 0, 0};
/// let rs = sdsl::rank_supports::RankSupportV::<sdsl::BitPatterns::P01>::new(&bv)?;
/// let result = rs.rank(4);
/// let expected = 2;
/// assert_eq!(result, expected);
/// ```
///
/// For further examples see [here](https://github.com/sdsl-rs/sdsl-rs/blob/master/examples/src/rank_supports/rank_support_v.rs).
///
/// # References
/// - Sebastiano Vigna:
///   Broadword Implementation of Rank/Select Queries.
///   WEA 2008: 154-168
pub struct RankSupportV<'a, BitPattern: common::bit_patterns::BitPattern> {
    // Dummy field so BitPattern is used, always None.
    _bp: Option<BitPattern>,
    // Dummy field to retain reference to bit vector.
    _bit_vector: Option<&'a super::bit_vector::BitVector>,
    ptr: common::VoidPtr,
    interface: Interface,
}

impl<'a, BitPattern: common::bit_patterns::BitPattern> RankSupportV<'a, BitPattern> {
    /// Construct a new rank structure.
    /// # Arguments
    /// * `bit_vector` - Bit vector.
    pub fn new(bit_vector: &'a super::bit_vector::BitVector) -> Result<Self> {
        let id = Self::id()?;
        let interface = Interface::new(&id)?;
        let ptr = (interface.create)(*bit_vector.ptr());

        Ok(Self {
            _bp: None,
            _bit_vector: Some(&bit_vector),
            ptr,
            interface,
        })
    }

    fn default() -> Result<Self> {
        let id = Self::id()?;
        let interface = Interface::new(&id)?;
        let ptr = (interface.create)(std::ptr::null());

        Ok(Self {
            _bp: None,
            _bit_vector: None,
            ptr,
            interface,
        })
    }

    /// Load from file.
    /// # Arguments
    /// * `path` - File path.
    pub fn from_file(path: &std::path::PathBuf) -> Result<Self> {
        let rs = Self::default()?;

        let path = path
            .to_str()
            .ok_or(format_err!("Failed to convert PathBuf into str."))?;
        let path = std::ffi::CString::new(path)?;

        (rs.interface.io.load_from_file)(rs.ptr, path.as_ptr());
        Ok(rs)
    }

    /// Get rank at index.
    /// # Arguments
    /// * `index` - An index in range $ [0, \mathrm{len}()) $.
    pub fn rank(&self, index: usize) -> usize {
        (self.interface.rank)(self.ptr, index)
    }

    /// The number of elements in the vector.
    pub fn len(&self) -> usize {
        (self.interface.len)(self.ptr)
    }
}

impl<'a, BitPattern: common::bit_patterns::BitPattern> common::io::IO
    for RankSupportV<'a, BitPattern>
{
    fn io(&self) -> &common::io::Interface {
        &self.interface.io
    }
}

impl<'a, BitPattern: common::bit_patterns::BitPattern> common::Ptr
    for RankSupportV<'a, BitPattern>
{
    fn ptr(&self) -> &common::VoidPtr {
        &self.ptr
    }
}

impl<'a, BitPattern: common::bit_patterns::BitPattern> common::Id for RankSupportV<'a, BitPattern> {
    fn id() -> Result<String> {
        let meta =
            Box::new(meta::rank_support_v::RankSupportVMeta::new()) as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        let id = sdsl_c::specification::get_id(&meta.c_code(&parameters_c_code)?)?;
        Ok(id)
    }
}

impl<'a, BitPattern: common::bit_patterns::BitPattern> common::Code
    for RankSupportV<'a, BitPattern>
{
    fn c_code() -> Result<String> {
        let meta =
            Box::new(meta::rank_support_v::RankSupportVMeta::new()) as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        Ok(meta.c_code(&parameters_c_code)?)
    }

    fn parameters_c_code() -> Result<Vec<String>> {
        Ok(vec![BitPattern::c_code()?])
    }
}

impl<'a, BitPattern: common::bit_patterns::BitPattern> Drop for RankSupportV<'a, BitPattern> {
    fn drop(&mut self) {
        (self.interface.drop)(self.ptr)
    }
}

#[derive(Clone)]
struct Interface {
    create: extern "C" fn(common::VoidPtr) -> common::VoidPtr,
    drop: extern "C" fn(common::VoidPtr),

    len: extern "C" fn(common::VoidPtr) -> usize,
    rank: extern "C" fn(common::VoidPtr, usize) -> usize,

    pub io: common::io::Interface,
    _lib: std::sync::Arc<sharedlib::Lib>,
}

impl Interface {
    pub fn new(id: &str) -> Result<Self> {
        let lib = sdsl_c::LIB.clone();
        let builder = sdsl_c::FunctionBuilder::new(Some("rank_support_v"), id, lib.clone());

        Ok(Self {
            create: builder.get("create")?,
            drop: builder.get("destroy")?,

            rank: builder.get("rank")?,
            len: builder.get("size")?,

            io: common::io::Interface::new(&id)?,
            _lib: lib.clone(),
        })
    }
}
