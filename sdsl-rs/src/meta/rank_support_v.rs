use crate::meta::common::{self, Code, Parameters};
use anyhow::Result;

pub struct RankSupportVMeta;

impl RankSupportVMeta {
    pub fn new() -> Self {
        Self {}
    }
}

impl common::Meta for RankSupportVMeta {
    fn file_specifications(
        &self,
        parameter_values: &Vec<String>,
        id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        let header = get_header_specification(&parameter_values, &id)?;
        let source = get_source_specification(&header, &id)?;

        let c_code = self.c_code(&parameter_values)?;
        let io_specifications = common::io::file_specifications(&c_code, None, &id)?;

        let mut specifications = vec![source, header];
        specifications.extend(io_specifications);
        Ok(specifications)
    }
}

fn get_header_specification(
    parameter_values: &Vec<String>,
    id: &str,
) -> Result<common::FileSpecification> {
    let template_file_name = std::path::PathBuf::from("rank_support_v.hpp");
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
    let template_file_name = std::path::PathBuf::from("rank_support_v.cpp");
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
        "#define RANK_SUPPORT_V_TEMPLATE {}",
        parameter_values.join(", ")
    );
    replacements.insert("#define RANK_SUPPORT_V_TEMPLATE 1, 1".to_string(), template);

    replacements.insert(
        "#define RANK_SUPPORT_V_ID _id".to_string(),
        format!("#define RANK_SUPPORT_V_ID _{}", id),
    );

    replacements
}

impl common::Path for RankSupportVMeta {
    fn path(&self) -> String {
        "sdsl::RankSupportV".to_string()
    }
}

impl common::Code for RankSupportVMeta {
    fn c_code(&self, parameter_values: &Vec<String>) -> Result<String> {
        let parameters = self.parameters();
        let parameter_values = common::c_sorted_parameters(&parameter_values, &parameters)?;
        Ok(format!(
            "sdsl::rank_support_v<{}>",
            parameter_values.join(", ")
        ))
    }
}

impl common::Parameters for RankSupportVMeta {
    fn parameters(&self) -> Vec<common::params::Parameter> {
        vec![common::params::Parameter::sdsl(0, false, 0)]
    }
}
