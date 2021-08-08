use crate::backend::sdsl_c;
use crate::meta;
use anyhow::{format_err, Result};

use crate::interface::common::{self, Code, Id};

/// A bit vector where each element is 1 bit.
///
/// # Example
/// ```ignore
/// let bv = sdsl::bit_vectors::BitVector::new(5, 1)?;
/// let result: Vec<_> = bv.iter().collect();
/// let expected = vec![1, 1, 1, 1, 1];
/// assert_eq!(result, expected);
/// ```
///
/// For further examples see [here](https://github.com/sdsl-rs/sdsl-rs/blob/master/examples/src/bit_vectors/bit_vector.rs).
pub struct BitVector {
    ptr: common::VoidPtr,
    interface: Interface,
}

impl BitVector {
    /// Construct a new bit vector.
    /// # Arguments
    /// * `size` - Number of elements.
    /// * `default_value` - Default values for elements initialization.
    pub fn new(size: usize, default_value: usize) -> Result<Self> {
        let id = Self::id()?;
        let interface = Interface::new(&id)?;
        let ptr = (interface.create)(size, default_value);

        Ok(Self { ptr, interface })
    }

    /// Load vector from file.
    /// # Arguments
    /// * `path` - File path.
    pub fn from_file(path: &std::path::PathBuf) -> Result<Self> {
        let vector = Self::new(1, 0)?;

        let path = path
            .to_str()
            .ok_or(format_err!("Failed to convert PathBuf into str."))?;
        let path = std::ffi::CString::new(path)?;

        (vector.interface.io.load_from_file)(vector.ptr, path.as_ptr());
        Ok(vector)
    }

    /// Returns true if the vector is empty, otherwise returns false.
    pub fn is_empty(&self) -> bool {
        (self.interface.is_empty)(self.ptr)
    }

    /// Resize the vector in terms of elements.
    /// # Arguments
    /// * `size` - Target number of elements.
    pub fn resize(&mut self, size: usize) {
        (self.interface.resize)(self.ptr, size)
    }

    /// The number of elements in the vector.
    pub fn len(&self) -> usize {
        (self.interface.len)(self.ptr)
    }

    /// Maximum size of the vector.
    pub fn max_size(&self) -> usize {
        (self.interface.max_size)(self.ptr)
    }

    /// The number of bits in the vector.
    pub fn bit_size(&self) -> usize {
        (self.interface.bit_size)(self.ptr)
    }

    /// Returns the size of the occupied bits of the vector.
    ///
    /// The capacity of a vector is greater or equal to the
    /// `bit_size()`.
    pub fn capacity(&self) -> usize {
        (self.interface.capacity)(self.ptr)
    }

    /// Constant pointer to the raw data of the vector.
    pub fn data(&self) -> common::VoidPtr {
        // TODO: Tie pointer lifetime to self.
        (self.interface.data)(self.ptr)
    }

    /// Get the integer value of the binary string of length `len` starting at position `index` in the vector.
    ///
    /// # Arguments
    /// * `index` - Starting index of the binary representation of the integer.
    /// * `len` - Length of the binary representation of the integer.
    ///
    /// # Example
    /// ```ignore
    /// //                          1, 2, 4, 8, 16
    /// let bv = sdsl::bit_vector! {1, 1, 0, 0, 1};
    /// let result = bv.get_int(0, 5);
    /// let expected = 19; // = 1 + 2 + 16
    /// assert_eq!(result, expected);
    /// ```
    pub fn get_int(&self, index: usize, len: u8) -> usize {
        (self.interface.get_int)(self.ptr, index, len)
    }

    /// Set the bits from position `index` to `index+len-1` to the binary representation of integer `value`.
    ///
    /// The bit at position `index` represents the least significant bit (lsb), and the bit at
    /// position `index+len-1` the most significant bit (msb) of `value`.
    /// # Arguments
    /// * `index` - Starting index of the binary representation of `value`.
    /// * `value` - The integer to store in the vector.
    /// * `len` - The length used to store `value` in the vector.
    pub fn set_int(&mut self, index: usize, value: usize, len: u8) {
        (self.interface.set_int)(self.ptr, index, value, len)
    }

    /// Get the i-th element of the vector.
    /// # Arguments
    /// * `index` - An index in range $ [0, \mathrm{len}()) $.
    pub fn get(&self, index: usize) -> u8 {
        (self.interface.get)(self.ptr, index)
    }

    /// Set the i-th element of the vector.
    /// # Arguments
    /// * `index` - An index in range $ [0, \mathrm{len}()) $.
    /// * `value` - New element value.
    pub fn set(&mut self, index: usize, value: usize) {
        (self.interface.set)(self.ptr, index, value)
    }

    /// Flip all bits.
    pub fn flip(&mut self) {
        (self.interface.flip)(self.ptr)
    }

    /// Returns an iterator over the vector values.
    pub fn iter(&self) -> common::VectorIterator<u8, Self> {
        common::VectorIterator::new(&self, self.len())
    }
}

impl common::util::Util for BitVector {
    fn util(&self) -> &common::util::Interface {
        &self.interface.util
    }
}

impl common::io::IO for BitVector {
    fn io(&self) -> &common::io::Interface {
        &self.interface.io
    }
}

impl common::Ptr for BitVector {
    fn ptr(&self) -> &common::VoidPtr {
        &self.ptr
    }
}

impl common::Id for BitVector {
    fn id() -> Result<String> {
        let meta = Box::new(meta::bit_vector::BitVectorMeta::new()) as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        let id = sdsl_c::specification::get_id(&meta.c_code(&parameters_c_code)?)?;
        Ok(id)
    }
}

impl common::Code for BitVector {
    fn c_code() -> Result<String> {
        let meta = Box::new(meta::bit_vector::BitVectorMeta::new()) as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        Ok(meta.c_code(&parameters_c_code)?)
    }

    fn parameters_c_code() -> Result<Vec<String>> {
        Ok(vec![])
    }
}

impl common::IterGet<u8> for BitVector {
    fn iter_get(&self, index: usize) -> u8 {
        (self.interface.get)(self.ptr, index)
    }
}

impl Drop for BitVector {
    fn drop(&mut self) {
        (self.interface.drop)(self.ptr)
    }
}

impl Clone for BitVector {
    fn clone(&self) -> Self {
        Self {
            ptr: (self.interface.clone)(self.ptr),
            interface: self.interface.clone(),
        }
    }
}

impl std::fmt::Debug for BitVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl PartialEq for BitVector {
    /// Equality operator for two vectors.
    ///
    /// Two int_vectors are equal if
    /// - capacities and sizes are equal and
    /// - width are equal and
    /// - the bits in the range `[0..bit_size()-1]` are equal.
    fn eq(&self, other: &Self) -> bool {
        (self.interface.equality)(self.ptr, other.ptr)
    }
}

impl Eq for BitVector {}

impl IntoIterator for BitVector {
    type Item = u8;
    type IntoIter = common::VectorIntoIterator<u8, BitVector>;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        common::VectorIntoIterator::new(self, len)
    }
}

#[derive(Clone)]
struct Interface {
    create: extern "C" fn(usize, usize) -> common::VoidPtr,
    drop: extern "C" fn(common::VoidPtr),
    clone: extern "C" fn(common::VoidPtr) -> common::VoidPtr,
    is_empty: extern "C" fn(common::VoidPtr) -> bool,

    resize: extern "C" fn(common::VoidPtr, usize),
    len: extern "C" fn(common::VoidPtr) -> usize,
    max_size: extern "C" fn(common::VoidPtr) -> usize,
    bit_size: extern "C" fn(common::VoidPtr) -> usize,
    capacity: extern "C" fn(common::VoidPtr) -> usize,

    data: extern "C" fn(common::VoidPtr) -> common::VoidPtr,

    get_int: extern "C" fn(common::VoidPtr, usize, u8) -> usize,
    set_int: extern "C" fn(common::VoidPtr, usize, usize, u8),
    get: extern "C" fn(common::VoidPtr, usize) -> u8,
    set: extern "C" fn(common::VoidPtr, usize, usize),

    equality: extern "C" fn(common::VoidPtr, common::VoidPtr) -> bool,

    flip: extern "C" fn(common::VoidPtr),

    pub io: common::io::Interface,
    util: common::util::Interface,
    _lib: std::sync::Arc<sharedlib::Lib>,
}

impl Interface {
    pub fn new(id: &str) -> Result<Self> {
        let lib = sdsl_c::LIB.clone();
        let builder = sdsl_c::FunctionBuilder::new(Some("bit_vector"), id, lib.clone());

        Ok(Self {
            create: builder.get("create")?,
            drop: builder.get("destroy")?,
            clone: builder.get("copy")?,
            is_empty: builder.get("empty")?,

            resize: builder.get("resize")?,
            len: builder.get("size")?,
            max_size: builder.get("max_size")?,
            bit_size: builder.get("bit_size")?,
            capacity: builder.get("capacity")?,

            data: builder.get("data")?,

            get_int: builder.get("get_int")?,
            set_int: builder.get("set_int")?,
            get: builder.get("get_element")?,
            set: builder.get("set_element")?,

            equality: builder.get("equality_operator")?,

            flip: builder.get("flip")?,

            io: common::io::Interface::new(&id)?,
            util: common::util::Interface::new(&id)?,
            _lib: lib.clone(),
        })
    }
}

/// Create a **BitVector** from a list of elements.
///
/// # Example
/// ```ignore
/// let bv = sdsl::bit_vector! {1, 1, 0, 1};
/// ```
#[macro_export(local_inner_macros)]
macro_rules! bit_vector {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(bit_vector!(@single $rest)),*]));

    ($($key:expr,)+) => { bit_vector!($($key),+) };
    ($($key:expr),*) => {
        {
            let _size = bit_vector!(@count $($key),*);
            let mut _vec = sdsl::bit_vectors::BitVector::new(_size, 0)?;
            let mut i = 0;
            $(
                _vec.set(i, $key);
                i += 1;
            )*
            _vec
        }
    };
}
