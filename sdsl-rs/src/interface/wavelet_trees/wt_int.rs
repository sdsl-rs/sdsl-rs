use crate::meta;
use crate::{backend::sdsl_c, interface::common::Ptr};
use anyhow::{format_err, Result};

use crate::interface::common::{self, Code, Id};

type Value = u64;
type Size = usize;

/// A wavelet tree class for integer sequences.
///
/// ## Space complexity
/// $\Order{n\log|\Sigma|}$ bits, where $n$ is the size of the vector for which the wavelet tree was built.
///
/// # Arguments
/// * `BitVector` - Underlying bitvector structure.
/// * `RankSupport1` - Rank support for pattern `1` on the bitvector.
/// * `SelectSupport1` - Select support for pattern `1` on the bitvector.
/// * `SelectSupport0` - Select support for pattern `0` on the bitvector.
///
/// # Example
///
/// ```ignore
/// let iv = sdsl::int_vector! {1, 1, 5, 1, 1, 2, 3, 4};
/// let wt = sdsl::wavelet_trees::WtInt::<sdsl::bit_vectors::BitVector>::from_int_vector(&iv)?;
///
/// let result = wt.get(2);
/// let expected = 5;
/// assert_eq!(result, expected);
/// ```
///
/// For further examples see [here](https://github.com/sdsl-rs/sdsl-rs/blob/master/examples/src/wt_int.rs).
pub struct WtInt<
    'a,
    BitVector = crate::bit_vectors::BitVector,
    RankSupport1 = crate::rank_supports::RankSupportV<'a, crate::bit_patterns::P1>,
    SelectSupport1 = crate::select_supports::SelectSupportMcl<'a, crate::bit_patterns::P1>,
    SelectSupport0 = crate::select_supports::SelectSupportMcl<'a, crate::bit_patterns::P0>,
> where
    BitVector: common::Code,
    RankSupport1: common::Code,
    SelectSupport1: common::Code,
    SelectSupport0: common::Code,
{
    // Dummy fields which are never used, always None. Included so that generic parameters are used.
    _bs: Option<BitVector>,
    _rs1: &'a Option<RankSupport1>,
    _ss1: &'a Option<SelectSupport1>,
    _ss0: &'a Option<SelectSupport0>,

    ptr: common::VoidPtr,
    interface: Interface<Value, Size>,
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0>
    WtInt<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0>
where
    BitVector: common::Code + 'a,
    RankSupport1: common::Code,
    SelectSupport1: common::Code,
    SelectSupport0: common::Code,
{
    /// Construct a wavelet tree from file.
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

    /// Construct a wavelet tree from a string.
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

    /// Construct a wavelet tree from an integer vector.
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

    /// Construct a wavelet tree from a bit vector.
    /// # Arguments
    /// * `bit_vector` - Bitvector.
    pub fn from_bit_vector(bit_vector: &crate::interface::bit_vector::BitVector) -> Result<Self> {
        let id = Self::id()?;
        let interface = Interface::new(&id)?;
        let ptr = (interface.from_bit_vector)(*bit_vector.ptr());
        let wt = Self::new(interface, ptr)?;
        Ok(wt)
    }

    fn new(interface: Interface<Value, Size>, ptr: common::VoidPtr) -> Result<Self> {
        Ok(Self {
            _bs: None,
            _rs1: &None,
            _ss1: &None,
            _ss0: &None,

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
    pub fn get(&self, index: usize) -> Value {
        (self.interface.get)(self.ptr, index)
    }

    /// Returns a count of the given symbol within the prefix $ [0, \mathrm{index}-1] $.
    ///
    /// The time complexity is $ \mathcal{O}(H_0) $ on average, where $ H_0 $ is the zero order entropy of the sequence.
    /// # Arguments
    /// * `index` - An index in range $ [0, \mathrm{len}()) $.
    /// * `symbol` - Symbol.
    pub fn rank(&self, index: usize, symbol: usize) -> usize {
        (self.interface.rank)(self.ptr, index, symbol)
    }

    /// Returns the symbol `wt[index]` and a count of its occurrences within the prefix $ [0, \mathrm{index}-1] $.
    ///
    /// The time complexity is $ \mathcal{O}(H_0) $ on average, where $ H_0 $ is the zero order entropy of the sequence.
    /// # Arguments
    /// * `index` - An index in range $ [0, \mathrm{len}()) $.
    pub fn inverse_select(&self, index: usize) -> (usize, usize) {
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
    pub fn select(&self, i: usize, symbol: usize) -> usize {
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
    pub fn interval_symbols(&self, start_index: usize, end_index: usize) -> IntervalSymbols<Value> {
        let result = (self.interface.interval_symbols)(self.ptr, start_index, end_index);
        IntervalSymbols {
            interval_alphabet_size: result.interval_alphabet_size,
            interval_symbols: common::array_from_c_array(result.cs, result.length),
            rank_symbols_lower: common::array_from_c_array(result.rank_c_i, result.length),
            rank_symbols_upper: common::array_from_c_array(result.rank_c_j, result.length),

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
    pub fn lex_count(&self, start_index: usize, end_index: usize, symbol: Value) -> LexCount {
        (self.interface.lex_count)(self.ptr, start_index, end_index, symbol)
    }

    /// Returns a count of symbols which are lexicographic smaller than `symbol` in [0..i-1].
    ///
    /// This method is only available for lex ordered tree strategies.
    ///
    /// # Arguments
    /// * `index` - Exclusive right bound of the range.
    /// * `symbol` - Symbol.
    pub fn lex_smaller_count(&self, index: usize, symbol: Value) -> LexSmallerCount {
        (self.interface.lex_smaller_count)(self.ptr, index, symbol)
    }

    /// Returns a count of the number of different symbols in the wavelet tree.
    pub fn alphabet_size(&self) -> usize {
        (self.interface.alphabet_size)(self.ptr)
    }

    /// This function searches points in the index interval [lb..rb] and value interval [vlb..vrb].
    ///
    /// # Arguments
    /// * `start_index` - Left bound of index interval (inclusive)
    /// * `end_index` - Right bound of index interval (inclusive)
    /// * `start_value` - Left bound of value interval (inclusive)
    /// * `end_value` - Right bound of value interval (inclusive)
    /// * `report` - When true, matching points are returned.
    pub fn range_search_2d(
        &self,
        start_index: usize,
        end_index: usize,
        start_value: Value,
        end_value: Value,
        report: bool,
    ) -> RangeSearch2D<Value> {
        let result = (self.interface.range_search_2d)(
            self.ptr,
            start_index,
            end_index,
            start_value,
            end_value,
            report,
        );
        RangeSearch2D {
            count_found_points: result.count_found_points,
            point_indexes: common::array_from_c_array(
                result.point_indexes,
                result.count_found_points,
            ),
            point_values: common::array_from_c_array(
                result.point_values,
                result.count_found_points,
            ),

            internal_results: result,
            interface: self.interface.clone(),
        }
    }

    /// Returns an iterator over the vector that was used in constructing the wavelet tree.
    pub fn iter(&self) -> common::VectorIterator<Value, Self> {
        common::VectorIterator::new(&self, self.len())
    }
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0> common::io::IO
    for WtInt<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0>
where
    BitVector: common::Code,
    RankSupport1: common::Code,
    SelectSupport1: common::Code,
    SelectSupport0: common::Code,
{
    fn io(&self) -> &common::io::Interface {
        &self.interface.io
    }
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0> common::Ptr
    for WtInt<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0>
where
    BitVector: common::Code,
    RankSupport1: common::Code,
    SelectSupport1: common::Code,
    SelectSupport0: common::Code,
{
    fn ptr(&self) -> &common::VoidPtr {
        &self.ptr
    }
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0> common::Id
    for WtInt<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0>
where
    BitVector: common::Code + 'a,
    RankSupport1: common::Code + 'a,
    SelectSupport1: common::Code + 'a,
    SelectSupport0: common::Code + 'a,
{
    fn id() -> Result<String> {
        let meta =
            Box::new(meta::wavelet_trees::wt_int::WtIntMeta::new()) as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        let id = sdsl_c::specification::get_id(&meta.c_code(&parameters_c_code)?)?;
        Ok(id)
    }
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0> common::Code
    for WtInt<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0>
where
    BitVector: common::Code,
    RankSupport1: common::Code + 'a,
    SelectSupport1: common::Code + 'a,
    SelectSupport0: common::Code + 'a,
{
    fn c_code() -> Result<String> {
        let meta =
            Box::new(meta::wavelet_trees::wt_int::WtIntMeta::new()) as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        Ok(meta.c_code(&parameters_c_code)?)
    }

    fn parameters_c_code() -> Result<Vec<String>> {
        Ok(vec![
            BitVector::c_code()?,
            RankSupport1::c_code()?,
            SelectSupport1::c_code()?,
            SelectSupport0::c_code()?,
        ])
    }
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0> common::IterGet<Value>
    for WtInt<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0>
where
    BitVector: common::Code,
    RankSupport1: common::Code,
    SelectSupport1: common::Code,
    SelectSupport0: common::Code,
{
    fn iter_get(&self, index: usize) -> Value {
        (self.interface.get)(self.ptr, index)
    }
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0> Drop
    for WtInt<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0>
where
    BitVector: common::Code,
    RankSupport1: common::Code,
    SelectSupport1: common::Code,
    SelectSupport0: common::Code,
{
    fn drop(&mut self) {
        (self.interface.drop)(self.ptr)
    }
}

impl<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0> Clone
    for WtInt<'a, BitVector, RankSupport1, SelectSupport1, SelectSupport0>
where
    BitVector: common::Code,
    RankSupport1: common::Code,
    SelectSupport1: common::Code,
    SelectSupport0: common::Code,
{
    fn clone(&self) -> Self {
        Self {
            _bs: None,
            _rs1: &None,
            _ss1: &None,
            _ss0: &None,

            ptr: (self.interface.clone)(self.ptr),
            interface: self.interface.clone(),
        }
    }
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

pub struct IntervalSymbols<'a, Value> {
    pub interval_alphabet_size: usize,
    pub interval_symbols: &'a [Value],
    pub rank_symbols_lower: &'a [u64],
    pub rank_symbols_upper: &'a [u64],

    internal_results: ResultIntervalSymbols<Value>,
    interface: Interface<Value, Size>,
}

impl<'a, Value> Drop for IntervalSymbols<'a, Value> {
    fn drop(&mut self) {
        (self.interface.free_result_interval_symbols)(
            self.internal_results.cs,
            self.internal_results.rank_c_i,
            self.internal_results.rank_c_j,
        )
    }
}

#[repr(C)]
struct ResultIntervalSymbols<Value> {
    interval_alphabet_size: usize,
    length: usize,
    cs: *const Value,
    rank_c_i: *const u64,
    rank_c_j: *const u64,
}

#[repr(C)]
struct ResultRangeSearch2D<Value> {
    count_found_points: usize,
    point_indexes: *const usize,
    point_values: *const Value,
}

pub struct RangeSearch2D<'a, Value> {
    pub count_found_points: usize,
    pub point_indexes: &'a [usize],
    pub point_values: &'a [Value],

    internal_results: ResultRangeSearch2D<Value>,
    interface: Interface<Value, Size>,
}

impl<'a, Value> Drop for RangeSearch2D<'a, Value> {
    fn drop(&mut self) {
        (self.interface.free_result_range_search_2d)(
            self.internal_results.point_indexes,
            self.internal_results.point_values,
        )
    }
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

    len: extern "C" fn(common::VoidPtr) -> Size,
    is_empty: extern "C" fn(common::VoidPtr) -> bool,
    get: extern "C" fn(common::VoidPtr, Size) -> Value,
    rank: extern "C" fn(common::VoidPtr, Size, Size) -> Size,
    inverse_select: extern "C" fn(common::VoidPtr, Size) -> common::Pair<Size, Size>,
    select: extern "C" fn(common::VoidPtr, Size, Size) -> Size,
    interval_symbols: extern "C" fn(common::VoidPtr, Size, Size) -> ResultIntervalSymbols<Value>,
    free_result_interval_symbols: extern "C" fn(*const Value, *const u64, *const u64),
    lex_count: extern "C" fn(common::VoidPtr, Size, Size, Value) -> LexCount,
    lex_smaller_count: extern "C" fn(common::VoidPtr, Size, Value) -> LexSmallerCount,
    range_search_2d: extern "C" fn(
        common::VoidPtr,
        Size,
        Size,
        Value,
        Value,
        bool,
    ) -> ResultRangeSearch2D<Value>,
    free_result_range_search_2d: extern "C" fn(*const usize, *const Value),
    alphabet_size: extern "C" fn(common::VoidPtr) -> Size,

    pub io: common::io::Interface,
    _lib: std::sync::Arc<sharedlib::Lib>,
}

impl<Value, Size> Interface<Value, Size> {
    pub fn new(id: &str) -> Result<Self> {
        let lib = sdsl_c::LIB.clone();
        let builder = sdsl_c::FunctionBuilder::new(Some("wt_int"), id, lib.clone());

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
            range_search_2d: builder.get("range_search_2d")?,
            free_result_range_search_2d: builder.get("free_result_range_search_2d")?,
            alphabet_size: builder.get("alphabet_size")?,

            io: common::io::Interface::new(&id)?,
            _lib: lib.clone(),
        })
    }
}
