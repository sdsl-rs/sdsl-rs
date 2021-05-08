use crate::meta::common;

pub struct IntVectorMeta;

impl IntVectorMeta {
    pub fn new() -> Self {
        Self {}
    }
}

impl common::Meta for IntVectorMeta {
    fn template_file_name(&self) -> std::path::PathBuf {
        std::path::PathBuf::from("int_vector.hpp")
    }

    fn replacements(
        &self,
        parameter_values: Option<&Vec<String>>,
        id: &str,
    ) -> std::collections::BTreeMap<String, String> {
        let mut replacements = maplit::btreemap! {};

        if let Some(parameter_values) = parameter_values {
            let template = format!("#define WT_INT_TEMPLATE {}", parameter_values.join(", "));
            replacements.insert("#define WT_INT_TEMPLATE".to_string(), template);
        }

        replacements.insert(
            "#define WT_INT_ID".to_string(),
            format!("#define WT_INT_ID _{}", id),
        );

        replacements
    }
}

impl common::Path for IntVectorMeta {
    fn path(&self) -> String {
        "sdsl::int_vector::IntVector".to_string()
    }
}

impl common::params::Parameters for IntVectorMeta {
    fn parameters(&self) -> Vec<common::params::Parameter> {
        vec![common::params::Parameter::integer(0, false)]
    }
}
