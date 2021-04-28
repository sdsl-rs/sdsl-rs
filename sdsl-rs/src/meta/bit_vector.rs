use crate::meta::common;

pub struct BitVectorMeta;

impl BitVectorMeta {
    pub fn new() -> Self {
        Self {}
    }
}

impl common::Meta for BitVectorMeta {
    fn template_file_name(&self) -> std::path::PathBuf {
        std::path::PathBuf::from("bit_vector.hpp")
    }

    fn replacements(
        &self,
        parameter_values: Option<&Vec<String>>,
        id: &str,
    ) -> std::collections::BTreeMap<String, String> {
        maplit::btreemap! {}
    }
}

impl common::Path for BitVectorMeta {
    fn path(&self) -> String {
        "sdsl::bit_vector::BitVector".to_string()
    }
}

impl common::params::Parameters for BitVectorMeta {
    fn parameters(&self) -> Vec<common::params::Parameter> {
        vec![]
    }
}
