use crate::meta::common::{self, Code, Parameters};
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
        parameter_values: &Vec<String>,
        id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        let header = get_header_specification(&parameter_values, &id)?;
        let source = get_source_specification(&header, &id)?;

        let c_code = self.c_code(&parameter_values)?;

        let util_specifications = common::util::file_specifications(&c_code, &id)?;
        let io_specifications = common::io::file_specifications(&c_code, Some(&c_code), &id)?;

        let mut specifications = vec![source, header];
        specifications.extend(util_specifications);
        specifications.extend(io_specifications);
        Ok(specifications)
    }
}

fn get_header_specification(
    parameter_values: &Vec<String>,
    id: &str,
) -> Result<common::FileSpecification> {
    let template_file_name = std::path::PathBuf::from("int_vector.hpp");
    let target_file_name = common::get_target_file_name(&template_file_name, &id)?;

    Ok(common::FileSpecification {
        replacements: get_header_replacements(&parameter_values, &id),
        template_file_name: template_file_name.clone(),
        target_file_name: target_file_name.clone(),
        c_file_type: common::CFileType::Hpp,
    })
}

fn get_source_specification(
    header: &common::FileSpecification,
    id: &str,
) -> Result<common::FileSpecification> {
    let template_file_name = std::path::PathBuf::from("int_vector.cpp");
    Ok(common::FileSpecification {
        replacements: get_source_replacements(&header.template_file_name, &header.target_file_name),
        template_file_name: template_file_name.clone(),
        target_file_name: common::get_target_file_name(&template_file_name, &id)?,
        c_file_type: common::CFileType::Cpp,
    })
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
    parameter_values: &Vec<String>,
    id: &str,
) -> std::collections::BTreeMap<String, String> {
    let mut replacements = maplit::btreemap! {};

    let template = format!(
        "#define INT_VECTOR_TEMPLATE {}",
        parameter_values.join(", ")
    );
    replacements.insert("#define INT_VECTOR_TEMPLATE".to_string(), template);

    replacements.insert(
        "#define INT_VECTOR_ID _id".to_string(),
        format!("#define INT_VECTOR_ID _{}", id),
    );

    replacements
}

impl common::Path for IntVectorMeta {
    fn path(&self) -> String {
        "sdsl::IntVector".to_string()
    }
}

impl common::Code for IntVectorMeta {
    fn c_code(&self, parameter_values: &Vec<String>) -> Result<String> {
        let parameters = self.parameters();
        let parameter_values = common::c_sorted_parameters(&parameter_values, &parameters)?;
        Ok(format!("sdsl::int_vector<{}>", parameter_values.join(", ")))
    }
}

impl common::Parameters for IntVectorMeta {
    fn parameters(&self) -> Vec<common::params::Parameter> {
        vec![common::params::Parameter::integer(0, false, 0)]
    }
}
