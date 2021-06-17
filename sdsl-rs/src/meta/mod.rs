use anyhow::Result;

pub mod bit_vector;
pub mod common;
pub mod int_vector;
pub mod rank_support_v;
pub mod rrr_vector;
pub mod select_support_mcl;
pub mod wt_huff;

pub fn get_all() -> Result<Vec<Box<dyn common::Meta>>> {
    Ok(vec![
        Box::new(int_vector::IntVectorMeta::new()) as Box<dyn common::Meta>,
        Box::new(bit_vector::BitVectorMeta::new()) as Box<dyn common::Meta>,
        Box::new(rrr_vector::RrrVectorMeta::new()) as Box<dyn common::Meta>,
        Box::new(rank_support_v::RankSupportVMeta::new()) as Box<dyn common::Meta>,
        Box::new(select_support_mcl::SelectSupportMclMeta::new()) as Box<dyn common::Meta>,
        Box::new(common::bit_patterns::P0Meta::new()) as Box<dyn common::Meta>,
        Box::new(common::bit_patterns::P1Meta::new()) as Box<dyn common::Meta>,
        Box::new(common::bit_patterns::P10Meta::new()) as Box<dyn common::Meta>,
        Box::new(common::bit_patterns::P01Meta::new()) as Box<dyn common::Meta>,
        Box::new(wt_huff::WtHuffMeta::new()) as Box<dyn common::Meta>,
    ])
}
