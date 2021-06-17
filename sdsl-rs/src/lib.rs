//! A Rust interface for the Succinct Data Structure Library ([SDSL-lite](https://github.com/simongog/sdsl-lite)).

mod backend;
mod interface;
mod meta;

pub use crate::backend::build;
pub use crate::interface::common::io::crate_export as io;
pub use crate::interface::common::util::crate_export as util;

pub mod bit_patterns {
    pub use crate::interface::common::bit_patterns::{P0, P01, P1, P10};
}

pub mod int_vectors {
    pub use crate::interface::int_vector::IntVector;
}

pub mod bit_vectors {
    pub use crate::interface::{bit_vector::BitVector, rrr_vector::RrrVector};
}

pub mod rank_supports {
    pub use crate::interface::rank_support_v::RankSupportV;
}

pub mod select_supports {
    pub use crate::interface::select_support_mcl::SelectSupportMcl;
}

pub mod wavelet_trees {
    pub use crate::interface::{byte_tree::ByteTree, wt_huff::WtHuff};
}
