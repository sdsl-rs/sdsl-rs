pub mod io;
pub mod util;

pub type VoidPtr = *mut libc::c_void;

pub trait Ptr {
    fn ptr(&self) -> &VoidPtr;
}

pub trait IterGet {
    fn iter_get(&self, index: usize) -> usize;
}

pub struct VectorIterator<'a, T: IterGet> {
    vector: &'a T,
    len: usize,
    index: usize,
}

impl<'a, T: IterGet> VectorIterator<'a, T> {
    pub fn new(vector: &'a T, len: usize) -> Self {
        Self {
            vector,
            len,
            index: 0,
        }
    }
}

impl<'a, T: IterGet> Iterator for VectorIterator<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let result = if self.index < self.len {
            Some(self.vector.iter_get(self.index))
        } else {
            None
        };
        self.index += 1;
        result
    }
}
