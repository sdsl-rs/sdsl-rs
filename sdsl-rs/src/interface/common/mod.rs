use anyhow::Result;
pub mod bit_patterns;
pub mod io;
pub mod util;

pub type VoidPtr = *const libc::c_void;

pub trait Ptr {
    fn ptr(&self) -> &VoidPtr;
}

pub trait Id: Code {
    fn id() -> Result<String>;
}

pub trait Code {
    fn c_code() -> Result<String>;
    fn parameters_c_code() -> Result<Vec<String>>;
}

pub trait IterGet<Value> {
    fn iter_get(&self, index: usize) -> Value;
}

pub struct VectorIterator<'a, Value, Iterable: IterGet<Value>> {
    _t: Option<Value>,
    vector: &'a Iterable,
    len: usize,
    index: usize,
}

impl<'a, Value, Iterable: IterGet<Value>> VectorIterator<'a, Value, Iterable> {
    pub fn new(vector: &'a Iterable, len: usize) -> Self {
        Self {
            _t: None,
            vector,
            len,
            index: 0,
        }
    }
}

impl<'a, Value, Iterable: IterGet<Value>> Iterator for VectorIterator<'a, Value, Iterable> {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        let result = if self.index < self.len {
            Some(self.vector.iter_get(self.index))
        } else {
            None
        };
        self.index = self.index + 1;
        result
    }
}

pub struct VectorIntoIterator<Value, Iterable: IterGet<Value>> {
    _t: Option<Value>,
    vector: Iterable,
    len: usize,
    index: usize,
}

impl<Value, Iterable: IterGet<Value>> VectorIntoIterator<Value, Iterable> {
    pub fn new(vector: Iterable, len: usize) -> Self {
        Self {
            _t: None,
            vector,
            len,
            index: 0,
        }
    }
}

impl<Value, Iterable: IterGet<Value>> Iterator for VectorIntoIterator<Value, Iterable> {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        let result = if self.index < self.len {
            Some(self.vector.iter_get(self.index))
        } else {
            None
        };
        self.index += 1;
        result
    }
}

#[repr(C)]
pub struct Pair<X, Y> {
    x: X,
    y: Y,
}

impl<X, Y> From<Pair<X, Y>> for (X, Y) {
    fn from(pair: Pair<X, Y>) -> (X, Y) {
        (pair.x, pair.y)
    }
}

pub fn array_from_c_array<'a, T>(c_array_ptr: *const T, length: usize) -> &'a [T] {
    unsafe { std::slice::from_raw_parts(c_array_ptr, length) }
}
