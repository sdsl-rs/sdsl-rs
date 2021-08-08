use anyhow::Result;

pub mod bit_vector;
pub mod rrr_vector;

pub fn get_metas() -> Result<Vec<Box<dyn crate::meta::common::Meta>>> {
    let metas = vec![
        Box::new(bit_vector::BitVectorMeta::new()) as Box<dyn crate::meta::common::Meta>,
        Box::new(rrr_vector::RrrVectorMeta::new()) as Box<dyn crate::meta::common::Meta>,
    ];
    Ok(metas)
}
