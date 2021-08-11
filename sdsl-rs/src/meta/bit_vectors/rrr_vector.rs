use crate::meta::common::{self, Code, Parameters};
use anyhow::Result;

pub struct RrrVectorMeta {
    parameters_default_meta: Vec<Box<dyn common::Meta>>,
}

impl RrrVectorMeta {
    pub fn new() -> Self {
        Self {
            parameters_default_meta: vec![],
        }
    }
}

impl common::Meta for RrrVectorMeta {
    fn file_specifications(
        &self,
        parameters_c_code: &Vec<String>,
        _parameters_file_specs: &Vec<Vec<common::FileSpecification>>,
        id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        let header = header_specification(&parameters_c_code, &id, &self)?;
        let source = source_specification(&header, &id)?;

        let bit_vector_meta = crate::meta::bit_vectors::bit_vector::BitVectorMeta::new();
        let bit_vector_specs = bit_vector_meta.file_specifications(&vec![], &vec![], &id)?;

        let c_code = self.c_code(&parameters_c_code)?;
        let io_specifications = common::io::file_specifications(&c_code, None, &id)?;

        let mut specifications = vec![source, header];
        specifications.extend(bit_vector_specs);
        specifications.extend(io_specifications);
        Ok(specifications)
    }
}

fn header_specification(
    parameters_c_code: &Vec<String>,
    id: &str,
    meta: &RrrVectorMeta,
) -> Result<common::FileSpecification> {
    let template_file_name = std::path::PathBuf::from("bit_vectors/rrr_vector.hpp");
    let target_file_name = common::get_target_file_name(&template_file_name, &id)?;

    Ok(common::FileSpecification {
        replacements: get_header_replacements(&parameters_c_code, &id, &meta)?,
        template_file_name: template_file_name.clone(),
        target_file_name: target_file_name.clone(),
        c_file_type: common::CFileType::Hpp,
    })
}

fn source_specification(
    header: &common::FileSpecification,
    id: &str,
) -> Result<common::FileSpecification> {
    let template_file_name = std::path::PathBuf::from("bit_vectors/rrr_vector.cpp");
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
    parameters_c_code: &Vec<String>,
    id: &str,
    meta: &RrrVectorMeta,
) -> Result<std::collections::BTreeMap<String, String>> {
    let mut replacements = maplit::btreemap! {};

    let parameters = meta.parameters_definitions();
    let parameters_c_code = common::c_sorted_parameters(&parameters_c_code, &parameters)?;
    replacements.insert(
        "#define RRR_VECTOR_TEMPLATE 63, sdsl::int_vector<>, 32".to_string(),
        format!(
            "#define RRR_VECTOR_TEMPLATE {}",
            parameters_c_code.join(", ")
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
        "sdsl::bit_vectors::RrrVector".to_string()
    }
}

impl common::Code for RrrVectorMeta {
    fn c_code(&self, parameters_c_code: &Vec<String>) -> Result<String> {
        let parameters = self.parameters_definitions();
        let parameters_c_code = common::c_sorted_parameters(&parameters_c_code, &parameters)?;
        Ok(format!(
            "sdsl::rrr_vector<{}>",
            parameters_c_code.join(", ")
        ))
    }
}

impl common::Parameters for RrrVectorMeta {
    fn parameters_definitions(&self) -> Vec<common::params::Parameter> {
        vec![
            common::params::Parameter::sdsl(0, false, 1),
            common::params::Parameter::integer(1, false, 0),
            common::params::Parameter::integer(2, false, 2),
        ]
    }

    fn parameters_default_c_code(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn parameters_default_meta(&self) -> &Vec<Box<dyn common::Meta>> {
        &self.parameters_default_meta
    }
}
