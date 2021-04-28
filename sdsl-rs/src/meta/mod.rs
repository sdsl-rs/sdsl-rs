use anyhow::Result;

pub mod bit_vector;
pub mod common;
pub mod int_vector;
pub mod wt_int;

pub fn get_all() -> Result<Vec<Box<dyn common::Meta>>> {
    Ok(vec![
        Box::new(int_vector::IntVectorMeta::new()) as Box<dyn common::Meta>,
        Box::new(bit_vector::BitVectorMeta::new()) as Box<dyn common::Meta>,
        Box::new(wt_int::WtIntMeta::new()) as Box<dyn common::Meta>,
    ])
}
