use crate::meta::common::{self, Code, Parameters};
use anyhow::Result;

pub struct IntVectorMeta {
    parameters_default_meta: Vec<Box<dyn common::Meta>>,
}

impl IntVectorMeta {
    pub fn new() -> Self {
        Self {
            parameters_default_meta: vec![],
        }
    }
}

impl common::Meta for IntVectorMeta {
    fn file_specifications(
        &self,
        parameters_c_code: &Vec<String>,
        _parameters_file_specs: &Vec<Vec<common::FileSpecification>>,
        id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        let header = get_header_specification(&parameters_c_code, &id)?;
        let source = get_source_specification(&header, &id)?;

        let c_code = self.c_code(&parameters_c_code)?;

        let util_specifications = common::util::file_specifications(&c_code, &id)?;
        let io_specifications = common::io::file_specifications(&c_code, Some(&c_code), &id)?;

        let mut specifications = vec![source, header];
        specifications.extend(util_specifications);
        specifications.extend(io_specifications);
        Ok(specifications)
    }
}

fn get_header_specification(
    parameters_c_code: &Vec<String>,
    id: &str,
) -> Result<common::FileSpecification> {
    let template_file_name = std::path::PathBuf::from("int_vector.hpp");
    let target_file_name = common::get_target_file_name(&template_file_name, &id)?;

    Ok(common::FileSpecification {
        replacements: get_header_replacements(&parameters_c_code, &id),
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
    parameters_c_code: &Vec<String>,
    id: &str,
) -> std::collections::BTreeMap<String, String> {
    let mut replacements = maplit::btreemap! {};

    let template = format!(
        "#define INT_VECTOR_TEMPLATE {}",
        parameters_c_code.join(", ")
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
        "sdsl::int_vectors::IntVector".to_string()
    }
}

impl common::Code for IntVectorMeta {
    fn c_code(&self, parameters_c_code: &Vec<String>) -> Result<String> {
        let parameters = self.parameters_definitions();
        let parameters_c_code = common::c_sorted_parameters(&parameters_c_code, &parameters)?;
        Ok(format!(
            "sdsl::int_vector<{}>",
            parameters_c_code.join(", ")
        ))
    }
}

impl common::Parameters for IntVectorMeta {
    fn parameters_definitions(&self) -> Vec<common::params::Parameter> {
        vec![common::params::Parameter::integer(0, false, 0)]
    }

    fn parameters_default_c_code(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn parameters_default_meta(&self) -> &Vec<Box<dyn common::Meta>> {
        &self.parameters_default_meta
    }
}
