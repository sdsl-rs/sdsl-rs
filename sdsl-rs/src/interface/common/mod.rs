pub mod util;

pub type VoidPtr = *mut libc::c_void;

pub trait Ptr {
    fn ptr(&self) -> &VoidPtr;
}
