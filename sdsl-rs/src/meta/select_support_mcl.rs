use crate::meta::common::{self, Code, Parameters};
use anyhow::Result;

pub struct SelectSupportMclMeta {
    parameters: Vec<Box<dyn common::Meta>>,
}

impl SelectSupportMclMeta {
    pub fn new() -> Self {
        Self { parameters: vec![] }
    }

    pub fn new_parameterized(parameters: Vec<Box<dyn common::Meta>>) -> Self {
        Self { parameters }
    }
}

impl common::Meta for SelectSupportMclMeta {
    fn file_specifications(
        &self,
        parameters_c_code: &Vec<String>,
        _parameters_file_specs: &Vec<Vec<common::FileSpecification>>,
        id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        let header = get_header_specification(&parameters_c_code, &id)?;
        let source = get_source_specification(&header, &id)?;

        let c_code = self.c_code(&parameters_c_code)?;
        let io_specifications = common::io::file_specifications(&c_code, None, &id)?;

        let mut specifications = vec![source, header];
        specifications.extend(io_specifications);
        Ok(specifications)
    }
}

fn get_header_specification(
    parameters_c_code: &Vec<String>,
    id: &str,
) -> Result<common::FileSpecification> {
    let template_file_name = std::path::PathBuf::from("select_support_mcl.hpp");
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
    let template_file_name = std::path::PathBuf::from("select_support_mcl.cpp");
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
        "#define SELECT_SUPPORT_MCL_TEMPLATE {}",
        parameters_c_code.join(", ")
    );
    replacements.insert(
        "#define SELECT_SUPPORT_MCL_TEMPLATE 1, 1".to_string(),
        template,
    );

    replacements.insert(
        "#define SELECT_SUPPORT_MCL_ID _id".to_string(),
        format!("#define SELECT_SUPPORT_MCL_ID _{}", id),
    );

    replacements
}

impl common::Path for SelectSupportMclMeta {
    fn path(&self) -> String {
        "sdsl::rank_supports::RankSupportV".to_string()
    }
}

impl common::Code for SelectSupportMclMeta {
    fn c_code(&self, parameters_c_code: &Vec<String>) -> Result<String> {
        let parameters = self.parameters();
        let parameters_c_code = common::c_sorted_parameters(&parameters_c_code, &parameters)?;
        Ok(format!(
            "sdsl::select_support_mcl<{}>",
            parameters_c_code.join(", ")
        ))
    }
}

impl common::Parameters for SelectSupportMclMeta {
    fn parameters(&self) -> Vec<common::params::Parameter> {
        vec![common::params::Parameter::sdsl(0, false, 0)]
    }

    fn default_parameters_c_code(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn parameters_meta(&self) -> &Vec<Box<dyn common::Meta>> {
        &self.parameters
    }
}
