use crate::backend::sdsl_c;
use crate::meta;
use anyhow::{format_err, Result};

use crate::interface::common::{self, Code, Id};

/// A generic vector for integers of width $ [1..64] $ bits.
///
/// This generic vector class can be used to generate a vector that contains integers of fixed width $ [1..64] $.
///
/// # Arguments
/// * `WIDTH` - Width of an integer. If set to `0` it is variable during runtime, otherwise fixed at compile time.
///
/// # Example
/// ```ignore
/// let mut iv = sdsl::int_vectors::IntVector::<0>::new(5, 42, Some(64))?;
/// iv.bit_resize(2 * iv.width() as usize);
///
/// let result: Vec<_> = iv.iter().collect();
/// let expected = vec![42, 42];
/// assert_eq!(result, expected);
/// ```
///
/// For further examples see [here](https://github.com/sdsl-rs/sdsl-rs/blob/master/examples/src/int_vector.rs).
pub struct IntVector<const WIDTH: u8> {
    ptr: common::VoidPtr,
    interface: Interface,
}

impl<const WIDTH: u8> IntVector<WIDTH> {
    /// Construct a new integer vector.
    /// # Arguments
    /// * `size` - Number of elements.
    /// * `default_value` - Default values for elements initialization.
    /// * `width` - The width of each integer. Must be specified if `WIDTH == 0`.
    pub fn new(size: usize, default_value: usize, width: Option<u8>) -> Result<Self> {
        assert!(
            (WIDTH == 0 && width.is_some()) || (WIDTH != 0 && width.is_none()),
            "Width argument must be specified iff WIDTH const generic value is 0."
        );
        let width = match width {
            Some(width) => width,
            None => WIDTH,
        };

        let id = Self::id()?;
        let interface = Interface::new(&id)?;
        let ptr = (interface.create)(size, default_value, width);

        Ok(Self { ptr, interface })
    }

    /// Load vector from file.
    /// # Arguments
    /// * `path` - File path.
    pub fn from_file(path: &std::path::PathBuf) -> Result<Self> {
        assert!(
            WIDTH == 0,
            "Generic const WIDTH must be zero when loading from file."
        );
        let int_vector = Self::new(1, 0, Some(64))?;

        let path = path
            .to_str()
            .ok_or(format_err!("Failed to convert PathBuf into str."))?;
        let path = std::ffi::CString::new(path)?;

        (int_vector.interface.io.load_from_file)(int_vector.ptr, path.as_ptr());
        Ok(int_vector)
    }

    /// Get the i-th element of the vector.
    /// # Arguments
    /// * `index` - An index in range $ [0, \mathrm{len}()) $.
    pub fn get(&self, index: usize) -> usize {
        (self.interface.get)(self.ptr, index)
    }

    /// Set the i-th element of the vector.
    /// # Arguments
    /// * `index` - An index in range $ [0, \mathrm{len}()) $.
    /// * `value` - New element value.
    pub fn set(&mut self, index: usize, value: usize) {
        (self.interface.set)(self.ptr, index, value)
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

    /// Resize the total vector in terms of bits.
    /// # Arguments
    /// * `size` - The size to resize the vector in terms of bits.
    pub fn bit_resize(&mut self, size: usize) {
        (self.interface.bit_resize)(self.ptr, size)
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

    /// Returns the width of the integers which are accessed via the `get(...)` method.
    pub fn width(&self) -> u8 {
        (self.interface.width)(self.ptr)
    }

    /// Sets the width of the integers which are accessed via the `get(...)` method, if `WIDTH` equals 0.
    ///
    /// This function does not bit resize each element in the vector.
    /// Rather, after using this function, the raw data of the vector will be interpreted differently.
    /// # Arguments
    /// * `width` - New width of the integers.
    pub fn set_width(&mut self, width: usize) -> Result<()> {
        if WIDTH != 0 {
            Err(format_err!(
                "WIDTH is non-zero. Width is therefore immutable."
            ))
        } else {
            Ok((self.interface.set_width)(self.ptr, width))
        }
    }

    /// Returns an iterator over the vector values.
    pub fn iter(&self) -> common::VectorIterator<usize, Self> {
        common::VectorIterator::new(&self, self.len())
    }
}

impl<const WIDTH: u8> common::util::Util for IntVector<WIDTH> {
    fn util(&self) -> &common::util::Interface {
        &self.interface.util
    }
}

impl<const WIDTH: u8> common::io::IO for IntVector<WIDTH> {
    fn io(&self) -> &common::io::Interface {
        &self.interface.io
    }
}

impl<const WIDTH: u8> common::Ptr for IntVector<WIDTH> {
    fn ptr(&self) -> &common::VoidPtr {
        &self.ptr
    }
}

impl<'a, const WIDTH: u8> common::Id for IntVector<WIDTH> {
    fn id() -> Result<String> {
        let meta = Box::new(meta::int_vector::IntVectorMeta::new()) as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        let id = sdsl_c::specification::get_id(&meta.c_code(&parameters_c_code)?)?;
        Ok(id)
    }
}

impl<'a, const WIDTH: u8> common::Code for IntVector<WIDTH> {
    fn c_code() -> Result<String> {
        let meta = Box::new(meta::int_vector::IntVectorMeta::new()) as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        Ok(meta.c_code(&parameters_c_code)?)
    }

    fn parameters_c_code() -> Result<Vec<String>> {
        Ok(vec![WIDTH.to_string()])
    }
}

impl<const WIDTH: u8> common::IterGet<usize> for IntVector<WIDTH> {
    fn iter_get(&self, index: usize) -> usize {
        (self.interface.get)(self.ptr, index)
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

impl<const WIDTH: u8> std::fmt::Debug for IntVector<WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<const WIDTH: u8> IntoIterator for IntVector<WIDTH> {
    type Item = usize;
    type IntoIter = common::VectorIntoIterator<usize, IntVector<WIDTH>>;

    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        common::VectorIntoIterator::new(self, len)
    }
}

#[derive(Clone)]
struct Interface {
    create: extern "C" fn(usize, usize, u8) -> common::VoidPtr,
    drop: extern "C" fn(common::VoidPtr),
    clone: extern "C" fn(common::VoidPtr) -> common::VoidPtr,
    is_empty: extern "C" fn(common::VoidPtr) -> bool,

    resize: extern "C" fn(common::VoidPtr, usize),
    bit_resize: extern "C" fn(common::VoidPtr, usize),
    len: extern "C" fn(common::VoidPtr) -> usize,
    max_size: extern "C" fn(common::VoidPtr) -> usize,
    bit_size: extern "C" fn(common::VoidPtr) -> usize,
    capacity: extern "C" fn(common::VoidPtr) -> usize,

    data: extern "C" fn(common::VoidPtr) -> common::VoidPtr,
    width: extern "C" fn(common::VoidPtr) -> u8,
    set_width: extern "C" fn(common::VoidPtr, usize),

    get: extern "C" fn(common::VoidPtr, usize) -> usize,
    set: extern "C" fn(common::VoidPtr, usize, usize),

    pub io: common::io::Interface,
    util: common::util::Interface,
    _lib: std::sync::Arc<sharedlib::Lib>,
}

impl Interface {
    pub fn new(id: &str) -> Result<Self> {
        let lib = sdsl_c::LIB.clone();
        let builder = sdsl_c::FunctionBuilder::new(Some("int_vector"), id, lib.clone());

        Ok(Self {
            create: builder.get("create")?,
            drop: builder.get("destroy")?,
            clone: builder.get("copy")?,
            is_empty: builder.get("empty")?,

            resize: builder.get("resize")?,
            bit_resize: builder.get("bit_resize")?,
            len: builder.get("size")?,
            max_size: builder.get("max_size")?,
            bit_size: builder.get("bit_size")?,
            capacity: builder.get("capacity")?,

            get: builder.get("get_element")?,
            set: builder.get("set_element")?,

            data: builder.get("data")?,
            width: builder.get("width")?,
            set_width: builder.get("set_width")?,

            io: common::io::Interface::new(&id)?,
            util: common::util::Interface::new(&id)?,
            _lib: lib.clone(),
        })
    }
}

/// Create a **IntVector** from a list of elements.
///
/// Elements at construction have 64 bit widths.
///
/// # Example
/// ```ignore
/// let mut iv = sdsl::int_vector! {1, 12, 3};
/// sdsl::util::bit_compress(&mut iv);
/// let result = iv.width();
/// let expected = 4;
/// assert_eq!(result, expected);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! int_vector {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(int_vector!(@single $rest)),*]));

    ($($key:expr,)+) => { int_vector!($($key),+) };
    ($($key:expr),*) => {
        {
            let _size = int_vector!(@count $($key),*);
            let mut _vec = sdsl::int_vectors::IntVector::<0>::new(_size, 0, Some(64))?;
            let mut i = 0;
            $(
                _vec.set(i, $key);
                i += 1;
            )*
            _vec
        }
    };
}
