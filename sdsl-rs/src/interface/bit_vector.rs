pub type BitVector = crate::interface::int_vector::IntVector<1>;

#[macro_export(local_inner_macros)]
macro_rules! bit_vector {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(bit_vector!(@single $rest)),*]));

    ($($key:expr,)+) => { bit_vector!($($key),+) };
    ($($key:expr),*) => {
        {
            let _size = bit_vector!(@count $($key),*);
            let mut _vec = sdsl::BitVector::new(_size, 0, None)?;
            let mut i = 0;
            $(
                _vec.set(i, $key);
                i += 1;
            )*
            _vec
        }
    };
}
