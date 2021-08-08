use crate::meta;
use crate::{backend::sdsl_c, interface::common::Ptr};
use anyhow::{format_err, Result};

use crate::interface::common::{self, Code, Id};
use crate::interface::wavelet_trees::layouts;

/// A Huffman-shaped wavelet tree.
///
/// A wavelet tree is built for a vector of characters over the byte alphabet
/// $\Sigma$. If you need a wavelet tree for a integer alphabet you should
/// use `sdsl::wavelet_trees::WtInt`.
/// The wavelet tree $wt$ consists of a tree of bitvectors and provides
/// three efficient methods:
///  - The "[]"-operator: `wt[i]` returns the i-th symbol of vector for
///    which the wavelet tree was build for.
/// - The rank method: `wt.rank(i,c)` returns the number of occurrences
///   of symbol $c$ in the prefix [0..i-1] in the vector for which the
///   wavelet tree was build for.
/// - The select method: `wt.select(j,c)` returns the index
///   $i\in [0..\mathrm{len}()-1]$ of the j-th occurrence of symbol $c$.
///
/// ## Space complexity
/// $n H_0 + 2|\Sigma|\log n$ bits, where $n$ is the size
/// of the vector the wavelet tree was build for.
///
/// # Arguments
/// * `BitVector` - Underlying bitvector structure.
/// * `RankSupport1` - Rank support for pattern `1` on the bitvector.
/// * `SelectSupport1` - Select support for pattern `1` on the bitvector.
/// * `SelectSupport0` - Select support for pattern `0` on the bitvector.
/// * `TreeStrategy` - Layout of the tree structure in memory.
///
/// # References
/// The idea of using a Huffman shaped wavelet was first mentioned on page 17
/// of the following technical report:
///
/// Veli Mäkinen and Gonzalo Navarro:
/// "Succinct Suffix Arrays based on Run-Length Encoding.",
/// <http://swp.dcc.uchile.cl/TR/2005/TR_DCC-2005-004.pdf>
///
/// # Example
///
/// ```ignore
/// let bv = sdsl::bit_vector! {1, 1, 0, 1};
/// let wt = sdsl::wavelet_trees::WtHuff::<>::from_bit_vector(&bv)?;
///
/// let result = wt.get_int(1, 3);
/// let expected = 5;
/// assert_eq!(result, expected);
/// ```
///
/// For further examples see [here](https://github.com/sdsl-rs/sdsl-rs/blob/master/examples/src/wavelet_trees/wt_huff.rs).
pub struct WtHuff<
    'a,
    BitVector = crate::bit_vectors::BitVector,
    RankSupport1 = crate::rank_supports::RankSupportV<'a, crate::bit_patterns::P1>,
    SelectSupport1 = crate::select_supports::SelectSupportMcl<'a, crate::bit_patterns::P1>,
    SelectSupport0 = crate::select_supports::SelectSupportMcl<'a, crate::bit_patterns::P0>,
    TreeStrategy = layouts::byte_tree::ByteTree,
> where
    BitVector: common::Code,
    RankSupport1: common::Code,
    SelectSupport1: common::Code,
    SelectSupport0: common::Code,
    TreeStrategy: layouts::common::TreeStrategy + common::Code,
{
    // Dummy fields which are never used, always None. Included so that generic parameters are used.
    _bs: Option<BitVector>,
    _rs1: &'a Option<RankSupport1>,
    _ss1: &'a Option<SelectSupport1>,
    _ss0: &'a Option<SelectSupport0>,
    _ts: Option<TreeStrategy>,

    ptr: common::VoidPtr,
    interface: Interface<TreeStrategy::Value, TreeStrategy::Size>,
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy>
    WtHuff<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy>
where
    BitVector: common::Code + 'a,
    RankSupport1: common::Code,
    SelectSupport1: common::Code,
    SelectSupport0: common::Code,
    TreeStrategy: layouts::common::TreeStrategy + common::Code + 'a,
{
    /// Construct a Huffman-shaped wavelet tree from file.
    /// # Arguments
    /// * `path` - File path.
    pub fn from_file(path: &std::path::PathBuf) -> Result<Self> {
        let path = path
            .to_str()
            .ok_or(format_err!("Failed to convert PathBuf into str."))?;
        let path = std::ffi::CString::new(path)?;

        let id = Self::id()?;
        let interface = Interface::new(&id)?;
        let ptr = (interface.from_file)(path.as_ptr());
        let wt = Self::new(interface, ptr)?;
        Ok(wt)
    }

    /// Construct a Huffman-shaped wavelet tree from a string.
    /// # Arguments
    /// * `string` - Data string.
    pub fn from_str(string: &str) -> Result<Self> {
        let id = Self::id()?;
        let interface = Interface::new(&id)?;
        let c_string = std::ffi::CString::new(string)?;
        let ptr = (interface.from_string)(c_string.as_ptr());
        let wt = Self::new(interface, ptr)?;
        Ok(wt)
    }

    /// Construct a Huffman-shaped wavelet tree from an integer vector.
    /// # Arguments
    /// * `int_vector` - Integer vector.
    pub fn from_int_vector<const WIDTH: u8>(
        int_vector: &crate::interface::int_vector::IntVector<WIDTH>,
    ) -> Result<Self> {
        let id = Self::id()?;
        let interface = Interface::new(&id)?;
        let ptr = (interface.from_int_vector)(*int_vector.ptr());
        let wt = Self::new(interface, ptr)?;
        Ok(wt)
    }

    /// Construct a Huffman-shaped wavelet tree from a bit vector.
    /// # Arguments
    /// * `bit_vector` - Bitvector.
    pub fn from_bit_vector(
        bit_vector: &crate::interface::bit_vectors::bit_vector::BitVector,
    ) -> Result<Self> {
        let id = Self::id()?;
        let interface = Interface::new(&id)?;
        let ptr = (interface.from_bit_vector)(*bit_vector.ptr());
        let wt = Self::new(interface, ptr)?;
        Ok(wt)
    }

    fn new(
        interface: Interface<TreeStrategy::Value, TreeStrategy::Size>,
        ptr: common::VoidPtr,
    ) -> Result<Self> {
        Ok(Self {
            _bs: None,
            _rs1: &None,
            _ss1: &None,
            _ss0: &None,
            _ts: None,

            ptr,
            interface,
        })
    }

    /// Returns the length of the original vector that was used in constructing the wavelet tree.
    pub fn len(&self) -> usize {
        (self.interface.len)(self.ptr)
    }

    /// Returns true if the wavelet tree contains no data, otherwise returns false.
    pub fn is_empty(&self) -> bool {
        (self.interface.is_empty)(self.ptr)
    }

    /// Get the i-th element of the original vector that was used in constructing the wavelet tree.
    /// # Arguments
    /// * `index` - An index in range $ [0, \mathrm{len}()) $.
    pub fn get(&self, index: usize) -> TreeStrategy::Value {
        (self.interface.get)(self.ptr, index)
    }

    /// Returns a count of the given symbol within the prefix $ [0, \mathrm{index}-1] $.
    ///
    /// The time complexity is $ \mathcal{O}(H_0) $ on average, where $ H_0 $ is the zero order entropy of the sequence.
    /// # Arguments
    /// * `index` - An index in range $ [0, \mathrm{len}()) $.
    /// * `symbol` - Symbol.
    pub fn rank(
        &self,
        index: TreeStrategy::Size,
        symbol: TreeStrategy::Value,
    ) -> TreeStrategy::Size {
        (self.interface.rank)(self.ptr, index, symbol)
    }

    /// Returns the symbol `wt[index]` and a count of its occurrences within the prefix $ [0, \mathrm{index}-1] $.
    ///
    /// The time complexity is $ \mathcal{O}(H_0) $ on average, where $ H_0 $ is the zero order entropy of the sequence.
    /// # Arguments
    /// * `index` - An index in range $ [0, \mathrm{len}()) $.
    pub fn inverse_select(
        &self,
        index: TreeStrategy::Size,
    ) -> (TreeStrategy::Value, TreeStrategy::Size) {
        let (rank, symbol) = (self.interface.inverse_select)(self.ptr, index).into();
        (symbol, rank)
    }

    /// Returns the index of the i-th occurrence of the given symbol in the supported vector.
    ///
    /// The time complexity is $ \mathcal{O}(H_0) $ on average, where $ H_0 $ is the zero order entropy of the sequence.
    /// Precondition: $ 1 \leq \mathrm{index} \leq \mathrm{rank}(\mathrm{len}(), \mathrm{symbol}) $.
    /// # Arguments
    /// * `i` - i-th symbol occurrence.
    /// * `symbol` - Symbol.
    pub fn select(&self, i: TreeStrategy::Size, symbol: TreeStrategy::Value) -> TreeStrategy::Size {
        (self.interface.select)(self.ptr, i, symbol)
    }

    /// For each symbol c in wt[i..j-1] get rank(i,c) and rank(j,c).
    ///
    /// The time complexity is $ \mathcal{O}(\min{\sigma, k \log \sigma}) $
    /// Precondition:
    ///
    /// # Arguments
    /// * `start_index` - The start index (inclusive) of the interval.
    /// * `end_index` - The end index (exclusive) of the interval.
    pub fn interval_symbols(
        &self,
        start_index: TreeStrategy::Size,
        end_index: TreeStrategy::Size,
    ) -> IntervalSymbols<TreeStrategy::Value, TreeStrategy::Size> {
        let result = (self.interface.interval_symbols)(self.ptr, start_index, end_index);
        IntervalSymbols {
            interval_alphabet_size: result.interval_alphabet_size,
            interval_symbols: common::array_from_c_array(result.cs, result.length.into()),
            rank_symbols_lower: common::array_from_c_array(result.rank_c_i, result.length.into()),
            rank_symbols_upper: common::array_from_c_array(result.rank_c_j, result.length.into()),

            internal_results: result,
            interface: self.interface.clone(),
        }
    }

    /// Returns a count of elements which are lexicographic smaller/greater than `symbol` in [i..j-1].
    ///
    /// This method is only available for lex ordered tree strategies.
    ///
    /// # Arguments
    /// * `start_index` - The start index (inclusive) of the interval.
    /// * `end_index` - The end index (exclusive) of the interval.
    /// * `symbol` - Symbol.
    pub fn lex_count(
        &self,
        start_index: TreeStrategy::Size,
        end_index: TreeStrategy::Size,
        symbol: TreeStrategy::Value,
    ) -> LexCount {
        assert!(
            TreeStrategy::LEX_ORDERED,
            "TreeStrategy is not lex ordered."
        );
        (self.interface.lex_count)(self.ptr, start_index, end_index, symbol)
    }

    /// Returns a count of symbols which are lexicographic smaller than `symbol` in [0..i-1].
    ///
    /// This method is only available for lex ordered tree strategies.
    ///
    /// # Arguments
    /// * `index` - Exclusive right bound of the range.
    /// * `symbol` - Symbol.
    pub fn lex_smaller_count(
        &self,
        index: TreeStrategy::Size,
        symbol: TreeStrategy::Value,
    ) -> LexSmallerCount {
        assert!(
            TreeStrategy::LEX_ORDERED,
            "TreeStrategy is not lex ordered."
        );
        (self.interface.lex_smaller_count)(self.ptr, index, symbol)
    }

    /// For a given symbol returns the next larger or equal symbol in the wavelet tree.
    /// Returns None if a valid symbol was not found.
    ///
    /// # Arguments
    /// * `symbol` - Symbol.
    pub fn symbol_gte(&self, symbol: TreeStrategy::Value) -> Option<TreeStrategy::Value> {
        let result = (self.interface.symbol_gte)(self.ptr, symbol);
        if result.found {
            Some(result.symbol)
        } else {
            None
        }
    }

    /// For a given symbol returns the next lesser or equal symbol in the wavelet tree.
    /// Returns None if a valid symbol was not found.
    ///
    /// # Arguments
    /// * `symbol` - Symbol.
    pub fn symbol_lte(&self, symbol: TreeStrategy::Value) -> Option<TreeStrategy::Value> {
        let result = (self.interface.symbol_lte)(self.ptr, symbol);
        if result.found {
            Some(result.symbol)
        } else {
            None
        }
    }

    /// Returns a count of the number of different symbols in the wavelet tree.
    pub fn alphabet_size(&self) -> TreeStrategy::Size {
        (self.interface.alphabet_size)(self.ptr)
    }

    /// Returns an iterator over the vector that was used in constructing the wavelet tree.
    pub fn iter(&self) -> common::VectorIterator<TreeStrategy::Value, Self> {
        common::VectorIterator::new(&self, self.len())
    }
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy> common::io::IO
    for WtHuff<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy>
where
    BitVector: common::Code,
    RankSupport1: common::Code,
    SelectSupport1: common::Code,
    SelectSupport0: common::Code,
    TreeStrategy: layouts::common::TreeStrategy + common::Code,
{
    fn io(&self) -> &common::io::Interface {
        &self.interface.io
    }
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy> common::Ptr
    for WtHuff<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy>
where
    BitVector: common::Code,
    RankSupport1: common::Code,
    SelectSupport1: common::Code,
    SelectSupport0: common::Code,
    TreeStrategy: layouts::common::TreeStrategy + common::Code,
{
    fn ptr(&self) -> &common::VoidPtr {
        &self.ptr
    }
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy> common::Id
    for WtHuff<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy>
where
    BitVector: common::Code + 'a,
    RankSupport1: common::Code + 'a,
    SelectSupport1: common::Code + 'a,
    SelectSupport0: common::Code + 'a,
    TreeStrategy: layouts::common::TreeStrategy + common::Code + 'a,
{
    fn id() -> Result<String> {
        let meta = Box::new(meta::wavelet_trees::wt_huff::WtHuffMeta::new())
            as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        let id = sdsl_c::specification::get_id(&meta.c_code(&parameters_c_code)?)?;
        Ok(id)
    }
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy> common::Code
    for WtHuff<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy>
where
    BitVector: common::Code,
    RankSupport1: common::Code + 'a,
    SelectSupport1: common::Code + 'a,
    SelectSupport0: common::Code + 'a,
    TreeStrategy: layouts::common::TreeStrategy + common::Code,
{
    fn c_code() -> Result<String> {
        let meta = Box::new(meta::wavelet_trees::wt_huff::WtHuffMeta::new())
            as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        Ok(meta.c_code(&parameters_c_code)?)
    }

    fn parameters_c_code() -> Result<Vec<String>> {
        Ok(vec![
            BitVector::c_code()?,
            RankSupport1::c_code()?,
            SelectSupport1::c_code()?,
            SelectSupport0::c_code()?,
            TreeStrategy::c_code()?,
        ])
    }
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy>
    common::IterGet<TreeStrategy::Value>
    for WtHuff<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy>
where
    BitVector: common::Code,
    RankSupport1: common::Code,
    SelectSupport1: common::Code,
    SelectSupport0: common::Code,
    TreeStrategy: layouts::common::TreeStrategy + common::Code,
{
    fn iter_get(&self, index: usize) -> TreeStrategy::Value {
        (self.interface.get)(self.ptr, index)
    }
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy> Drop
    for WtHuff<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy>
where
    BitVector: common::Code,
    RankSupport1: common::Code,
    SelectSupport1: common::Code,
    SelectSupport0: common::Code,
    TreeStrategy: layouts::common::TreeStrategy + common::Code,
{
    fn drop(&mut self) {
        (self.interface.drop)(self.ptr)
    }
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy> Clone
    for WtHuff<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0, TreeStrategy>
where
    BitVector: common::Code,
    RankSupport1: common::Code,
    SelectSupport1: common::Code,
    SelectSupport0: common::Code,
    TreeStrategy: layouts::common::TreeStrategy + common::Code,
{
    fn clone(&self) -> Self {
        Self {
            _bs: None,
            _rs1: &None,
            _ss1: &None,
            _ss0: &None,
            _ts: None,

            ptr: (self.interface.clone)(self.ptr),
            interface: self.interface.clone(),
        }
    }
}

#[repr(C)]
struct SymbolGte<Value> {
    pub found: bool,
    pub symbol: Value,
}

#[repr(C)]
struct SymbolLte<Value> {
    pub found: bool,
    pub symbol: Value,
}

#[repr(C)]
pub struct LexCount {
    pub rank: usize,
    pub count_smaller_symbols: usize,
    pub count_greater_symbols: usize,
}

#[repr(C)]
pub struct LexSmallerCount {
    pub rank: usize,
    pub count_smaller_symbols: usize,
}

pub struct IntervalSymbols<'a, Value, Size> {
    pub interval_alphabet_size: Size,
    pub interval_symbols: &'a [Value],
    pub rank_symbols_lower: &'a [u64],
    pub rank_symbols_upper: &'a [u64],

    internal_results: ResultIntervalSymbols<Value, Size>,
    interface: Interface<Value, Size>,
}

impl<'a, Value, Size> Drop for IntervalSymbols<'a, Value, Size> {
    fn drop(&mut self) {
        (self.interface.free_result_interval_symbols)(
            self.internal_results.cs,
            self.internal_results.rank_c_i,
            self.internal_results.rank_c_j,
        )
    }
}

#[repr(C)]
struct ResultIntervalSymbols<Value, Size> {
    interval_alphabet_size: Size,
    length: Size,
    cs: *const Value,
    rank_c_i: *const u64,
    rank_c_j: *const u64,
}

#[derive(Clone)]
struct Interface<Value, Size> {
    create: extern "C" fn() -> common::VoidPtr,
    from_file: extern "C" fn(*const std::os::raw::c_char) -> common::VoidPtr,
    from_string: extern "C" fn(*const std::os::raw::c_char) -> common::VoidPtr,
    from_int_vector: extern "C" fn(common::VoidPtr) -> common::VoidPtr,
    from_bit_vector: extern "C" fn(common::VoidPtr) -> common::VoidPtr,
    drop: extern "C" fn(common::VoidPtr),
    clone: extern "C" fn(common::VoidPtr) -> common::VoidPtr,

    len: extern "C" fn(common::VoidPtr) -> usize,
    is_empty: extern "C" fn(common::VoidPtr) -> bool,
    get: extern "C" fn(common::VoidPtr, usize) -> Value,
    rank: extern "C" fn(common::VoidPtr, Size, Value) -> Size,
    inverse_select: extern "C" fn(common::VoidPtr, Size) -> common::Pair<Size, Value>,
    select: extern "C" fn(common::VoidPtr, Size, Value) -> Size,
    interval_symbols:
        extern "C" fn(common::VoidPtr, Size, Size) -> ResultIntervalSymbols<Value, Size>,
    free_result_interval_symbols: extern "C" fn(*const Value, *const u64, *const u64),
    lex_count: extern "C" fn(common::VoidPtr, Size, Size, Value) -> LexCount,
    lex_smaller_count: extern "C" fn(common::VoidPtr, Size, Value) -> LexSmallerCount,
    symbol_gte: extern "C" fn(common::VoidPtr, Value) -> SymbolGte<Value>,
    symbol_lte: extern "C" fn(common::VoidPtr, Value) -> SymbolLte<Value>,
    alphabet_size: extern "C" fn(common::VoidPtr) -> Size,

    pub io: common::io::Interface,
    _lib: std::sync::Arc<sharedlib::Lib>,
}

impl<Value, Size> Interface<Value, Size> {
    pub fn new(id: &str) -> Result<Self> {
        let lib = sdsl_c::LIB.clone();
        let builder = sdsl_c::FunctionBuilder::new(Some("wt_huff"), id, lib.clone());

        Ok(Self {
            create: builder.get("create")?,
            from_file: builder.get("from_file")?,
            from_string: builder.get("from_string")?,
            from_int_vector: builder.get("from_int_vector")?,
            from_bit_vector: builder.get("from_bit_vector")?,
            drop: builder.get("destroy")?,
            clone: builder.get("copy")?,

            len: builder.get("size")?,
            is_empty: builder.get("empty")?,
            get: builder.get("get_element")?,
            rank: builder.get("rank")?,
            inverse_select: builder.get("inverse_select")?,
            select: builder.get("select")?,
            interval_symbols: builder.get("interval_symbols")?,
            free_result_interval_symbols: builder.get("free_result_interval_symbols")?,
            lex_count: builder.get("lex_count")?,
            lex_smaller_count: builder.get("lex_smaller_count")?,
            symbol_gte: builder.get("symbol_gte")?,
            symbol_lte: builder.get("symbol_lte")?,
            alphabet_size: builder.get("alphabet_size")?,

            io: common::io::Interface::new(&id)?,
            _lib: lib.clone(),
        })
    }
}
