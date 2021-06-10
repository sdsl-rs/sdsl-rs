//! A Rust interface for the Succinct Data Structure Library ([SDSL-lite](https://github.com/simongog/sdsl-lite)).

mod backend;
mod interface;
mod meta;

pub use crate::backend::build;
pub use crate::interface::{
    bit_vector::BitVector, common::bit_patterns, common::io, common::util, int_vector::IntVector,
    rank_support_v::RankSupportV, rrr_vector::RrrVector, select_support_mcl::SelectSupportMcl,
};
