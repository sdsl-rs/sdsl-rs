use crate::meta::common;
use anyhow::{format_err, Result};

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
        let source_template_file_name = std::path::PathBuf::from("int_vector.cpp");
        let source = common::FileSpecification {
            replacements: std::collections::BTreeMap::<String, String>::new(),
            template_file_name: source_template_file_name.clone(),
            target_file_name: get_target_file_name(&source_template_file_name, &id)?,
            c_file_type: common::CFileType::Cpp,
        };

        let header_template_file_name = std::path::PathBuf::from("int_vector.hpp");
        let header = common::FileSpecification {
            replacements: get_header_replacements(&parameter_values, &id),
            template_file_name: header_template_file_name.clone(),
            target_file_name: get_target_file_name(&header_template_file_name, &id)?,
            c_file_type: common::CFileType::Hpp,
        };

        Ok(vec![source, header])
    }
}

fn get_target_file_name(
    template_file_name: &std::path::PathBuf,
    id: &str,
) -> Result<std::path::PathBuf> {
    let stem = template_file_name
        .file_stem()
        .and_then(|s| s.to_str().to_owned())
        .ok_or(format_err!(
            "Failed to find stem for file: {}",
            template_file_name.display()
        ))?;
    let extension = template_file_name
        .extension()
        .and_then(|s| s.to_str().to_owned())
        .ok_or(format_err!(
            "Failed to find extension for file: {}",
            template_file_name.display()
        ))?;
    let target_file_name = format!(
        "{stem}_{id}.{extension}",
        stem = stem,
        id = id,
        extension = extension
    );
    Ok(std::path::PathBuf::from(target_file_name))
}

fn get_header_replacements(
    parameter_values: &Option<&Vec<String>>,
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
