use crate::meta::common::{self, Code, Parameters};
use anyhow::Result;

pub struct RrrVectorMeta;

impl RrrVectorMeta {
    pub fn new() -> Self {
        Self {}
    }
}

impl common::Meta for RrrVectorMeta {
    fn file_specifications(
        &self,
        parameter_values: &Vec<String>,
        id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        let header = header_specification(&parameter_values, &id, &self)?;
        let source = source_specification(&header, &id)?;

        let bit_vector_meta = crate::meta::bit_vector::BitVectorMeta::new();
        let bit_vector_specs = bit_vector_meta.file_specifications(&Vec::<_>::new(), &id)?;

        let c_code = self.c_code(&parameter_values)?;
        let io_specifications = common::io::file_specifications(&c_code, None, &id)?;

        let mut specifications = vec![source, header];
        specifications.extend(bit_vector_specs);
        specifications.extend(io_specifications);
        Ok(specifications)
    }
}

fn header_specification(
    parameter_values: &Vec<String>,
    id: &str,
    meta: &RrrVectorMeta,
) -> Result<common::FileSpecification> {
    let template_file_name = std::path::PathBuf::from("rrr_vector.hpp");
    let target_file_name = common::get_target_file_name(&template_file_name, &id)?;

    Ok(common::FileSpecification {
        replacements: get_header_replacements(&parameter_values, &id, &meta)?,
        template_file_name: template_file_name.clone(),
        target_file_name: target_file_name.clone(),
        c_file_type: common::CFileType::Hpp,
    })
}

fn source_specification(
    header: &common::FileSpecification,
    id: &str,
) -> Result<common::FileSpecification> {
    let template_file_name = std::path::PathBuf::from("rrr_vector.cpp");
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
    meta: &RrrVectorMeta,
) -> Result<std::collections::BTreeMap<String, String>> {
    let mut replacements = maplit::btreemap! {};

    let parameters = meta.parameters();
    let parameter_values = common::c_sorted_parameters(&parameter_values, &parameters)?;
    replacements.insert(
        "#define RRR_VECTOR_TEMPLATE 63, sdsl::int_vector<>, 32".to_string(),
        format!(
            "#define RRR_VECTOR_TEMPLATE {}",
            parameter_values.join(", ")
        ),
    );

    replacements.insert(
        "#define RRR_VECTOR_ID _id".to_string(),
        format!("#define RRR_VECTOR_ID _{}", id),
    );

    Ok(replacements)
}

impl common::Path for RrrVectorMeta {
    fn path(&self) -> String {
        "sdsl::RrrVector".to_string()
    }
}

impl common::Code for RrrVectorMeta {
    fn c_code(&self, parameter_values: &Vec<String>) -> Result<String> {
        let parameters = self.parameters();
        let parameter_values = common::c_sorted_parameters(&parameter_values, &parameters)?;
        Ok(format!("sdsl::rrr_vector<{}>", parameter_values.join(", ")))
    }
}

impl common::Parameters for RrrVectorMeta {
    fn parameters(&self) -> Vec<common::params::Parameter> {
        vec![
            common::params::Parameter::sdsl(0, false, 1),
            common::params::Parameter::integer(1, false, 0),
            common::params::Parameter::integer(2, false, 2),
        ]
    }
}
