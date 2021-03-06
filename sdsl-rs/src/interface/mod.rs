pub mod bit_vectors;
pub mod common;
pub mod int_vector;
pub mod rank_support_v;
pub mod select_support_mcl;
pub mod wavelet_trees;

pub mod crate_export {
    pub use crate::interface::bit_vectors::crate_export as bit_vectors;
    pub use crate::interface::common::io::crate_export as io;
    pub use crate::interface::common::util::crate_export as util;
    pub use crate::interface::wavelet_trees::crate_export as wavelet_trees;

    pub mod bit_patterns {
        pub use crate::interface::common::bit_patterns::{P0, P01, P1, P10};
    }

    pub mod int_vectors {
        pub use crate::interface::int_vector::IntVector;
    }

    pub mod rank_supports {
        pub use crate::interface::rank_support_v::RankSupportV;
    }

    pub mod select_supports {
        pub use crate::interface::select_support_mcl::SelectSupportMcl;
    }
}
