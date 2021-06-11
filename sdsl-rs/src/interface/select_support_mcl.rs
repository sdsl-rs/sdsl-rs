use crate::backend::sdsl_c;
use crate::meta;
use anyhow::{format_err, Result};

use crate::interface::common::{self, Id, ParameterValues, Ptr};

/// A class supporting constant time select queries.
///
/// ## Space usage
/// The space usage of the data structure depends on the number of $m$ of ones in the
/// original bitvector $b$. We store the position of every 4096-th set bit
/// (called L1-sampled bits) of $b$.
/// This takes in the worst case $ \frac{m}{4096} \log{n} \leq \frac{n}{64} $ bits.
///
/// Next, (1) if the distance of two adjacent L1-sampled bits $b[i]$ and $b[j]$
/// is greater or equal than $\log^4 n$, then
/// we store each of the 4096 positions of the set $b$ in [i..j-1] with
/// $\log{n}$ bits. This results in at most
/// \$ \frac{4096\cdot \log n}{\log^4 n}=\frac{4096}{\log^3 n}\$ bits per bit.
/// For a bitvector of 4GB, i.e. $ \log n = 35 $ we get about 0.01 bits per bit.
/// If the $j-i+1 < \log^4 n$ then
/// (2) we store the relative position of every $64$th set bit (called L2-sampled bits)
/// in b[i..j-1] in at most $4\log\log n$ bits per L2-sampled bits.
/// An pessimistic upper bound for the space would be
/// $ \frac{4\log\log n}{64} \leq \frac{24}{64} = 0.375 $ bit per
/// bit (since $\log\log n\leq 6$. It is very pessimistic, since we store
/// the relative position in $\log\log(j-i+1)\leq \log\log n$ bits.
///
/// # Arguments
/// * `BitPattern` - Bit pattern `0`,`1`,`10`,`01` supported by select query.
///
/// # Example
/// ```ignore
/// let bv = sdsl::bit_vector! {0, 1, 0, 1, 0, 0, 0};
/// let ss = sdsl::select_supports::SelectSupportMcl::<sdsl::BitPatterns::P1>::new(&bv)?;
/// let result = ss.select(2);
/// let expected = 3;
/// assert_eq!(result, expected);
/// ```
///
/// For further examples see [here](https://github.com/sdsl-rs/sdsl-rs/blob/master/examples/src/select_support_mcl.rs).
///
/// # References
/// - David Clark:
///   PhD Thesis: Compact Pat Trees
///   University of Waterloo, 1996 (Section 2.2.2).
///   http://www.nlc-bnc.ca/obj/s4/f2/dsk3/ftp04/nq21335.pdf
pub struct SelectSupportMcl<'a, BitPattern: common::bit_patterns::BitPattern> {
    // Dummy field so BitPattern is used, always None.
    _bp: Option<BitPattern>,
    // Dummy field to retain reference to bit vector.
    _bit_vector: Option<&'a super::bit_vector::BitVector>,
    ptr: common::VoidPtr,
    interface: Interface,
}

impl<'a, BitPattern: common::bit_patterns::BitPattern> SelectSupportMcl<'a, BitPattern> {
    /// Construct a new select support structure.
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

    /// Returns the position of the i-th bit pattern instance in the bit vector.
    /// # Arguments
    /// * `index` - An index within the range of the supported bit vector.
    pub fn select(&self, index: usize) -> usize {
        (self.interface.select)(self.ptr, index)
    }
}

impl<'a, BitPattern: common::bit_patterns::BitPattern> common::io::IO
    for SelectSupportMcl<'a, BitPattern>
{
    fn io(&self) -> &common::io::Interface {
        &self.interface.io
    }
}

impl<'a, BitPattern: common::bit_patterns::BitPattern> common::Ptr
    for SelectSupportMcl<'a, BitPattern>
{
    fn ptr(&self) -> &common::VoidPtr {
        &self.ptr
    }
}

impl<'a, BitPattern: common::bit_patterns::BitPattern> common::Id
    for SelectSupportMcl<'a, BitPattern>
{
    fn id() -> Result<String> {
        let meta = Box::new(meta::select_support_mcl::SelectSupportMclMeta::new())
            as Box<dyn meta::common::Meta>;
        let parameter_values = Self::parameter_values()?;
        let id = sdsl_c::specification::get_id(&meta.c_code(&parameter_values)?)?;
        Ok(id)
    }
}

impl<'a, BitPattern: common::bit_patterns::BitPattern> common::Code
    for SelectSupportMcl<'a, BitPattern>
{
    fn c_code() -> Result<String> {
        let meta = Box::new(meta::select_support_mcl::SelectSupportMclMeta::new())
            as Box<dyn meta::common::Meta>;
        let parameter_values = Self::parameter_values()?;
        Ok(meta.c_code(&parameter_values)?)
    }
}

impl<'a, BitPattern: common::bit_patterns::BitPattern> common::ParameterValues
    for SelectSupportMcl<'a, BitPattern>
{
    fn parameter_values() -> Result<Vec<String>> {
        Ok(vec![BitPattern::c_code()?])
    }
}

impl<'a, BitPattern: common::bit_patterns::BitPattern> Drop for SelectSupportMcl<'a, BitPattern> {
    fn drop(&mut self) {
        (self.interface.drop)(self.ptr)
    }
}

#[derive(Clone)]
struct Interface {
    create: extern "C" fn(common::VoidPtr) -> common::VoidPtr,
    drop: extern "C" fn(common::VoidPtr),

    select: extern "C" fn(common::VoidPtr, usize) -> usize,

    pub io: common::io::Interface,
    _lib: std::sync::Arc<sharedlib::Lib>,
}

impl Interface {
    pub fn new(id: &str) -> Result<Self> {
        let lib = sdsl_c::LIB.clone();
        let builder = sdsl_c::FunctionBuilder::new(Some("select_support_mcl"), id, lib.clone());

        Ok(Self {
            create: builder.get("create")?,
            drop: builder.get("destroy")?,

            select: builder.get("select")?,

            io: common::io::Interface::new(&id)?,
            _lib: lib.clone(),
        })
    }
}
