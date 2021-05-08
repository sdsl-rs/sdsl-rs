use anyhow::Result;

pub mod common;
pub mod int_vector;
// pub mod wt_int;

pub fn get_all() -> Result<Vec<Box<dyn common::Meta>>> {
    Ok(vec![
        Box::new(int_vector::IntVectorMeta::new()) as Box<dyn common::Meta>,
        // Box::new(wt_int::WtIntMeta::new()) as Box<dyn common::Meta>,
    ])
}
