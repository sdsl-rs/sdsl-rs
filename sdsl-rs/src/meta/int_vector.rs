use crate::meta::common;
use anyhow::Result;

pub struct IntVectorMeta;

impl IntVectorMeta {
    pub fn new() -> Self {
        Self {}
    }
}

impl common::Meta for IntVectorMeta {
    fn file_specifications(
        &self,
        parameter_values: &Option<&Vec<String>>,
        id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        let header_template_file_name = std::path::PathBuf::from("int_vector.hpp");
        let header_target_file_name =
            common::get_target_file_name(&header_template_file_name, &id)?;

        let header = common::FileSpecification {
            replacements: get_header_replacements(&parameter_values, &id),
            template_file_name: header_template_file_name.clone(),
            target_file_name: header_target_file_name.clone(),
            c_file_type: common::CFileType::Hpp,
        };

        let source_template_file_name = std::path::PathBuf::from("int_vector.cpp");
        let source = common::FileSpecification {
            replacements: get_source_replacements(
                &header_template_file_name,
                &header_target_file_name,
            ),
            template_file_name: source_template_file_name.clone(),
            target_file_name: common::get_target_file_name(&source_template_file_name, &id)?,
            c_file_type: common::CFileType::Cpp,
        };

        Ok(vec![source, header])
    }
}

fn get_source_replacements(
    header_template_file_name: &std::path::PathBuf,
    header_target_file_name: &std::path::PathBuf,
) -> std::collections::BTreeMap<String, String> {
    maplit::btreemap! {
        format!("#include \"{}\"", header_template_file_name.display()) => format!("#include \"{}\"", header_target_file_name.display())
    }
}

fn get_header_replacements(
    parameter_values: &Option<&Vec<String>>,
    id: &str,
) -> std::collections::BTreeMap<String, String> {
    let mut replacements = maplit::btreemap! {};

    if let Some(parameter_values) = parameter_values {
        let template = format!(
            "#define INT_VECTOR_TEMPLATE {}",
            parameter_values.join(", ")
        );
        replacements.insert("#define INT_VECTOR_TEMPLATE".to_string(), template);
    }

    replacements.insert(
        "#define INT_VECTOR_ID _id".to_string(),
        format!("#define INT_VECTOR_ID _{}", id),
    );

    replacements
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
