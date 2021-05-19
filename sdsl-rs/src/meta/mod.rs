use anyhow::Result;

pub mod bit_vector;
pub mod common;
pub mod int_vector;
pub mod rrr_vector;

pub fn get_all() -> Result<Vec<Box<dyn common::Meta>>> {
    Ok(vec![
        Box::new(int_vector::IntVectorMeta::new()) as Box<dyn common::Meta>,
        Box::new(rrr_vector::RrrVectorMeta::new()) as Box<dyn common::Meta>,
    ])
}
