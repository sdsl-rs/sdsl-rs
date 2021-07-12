use crate::interface::common::ParametersCCode;
use crate::meta::common;
use anyhow::{format_err, Result};

pub struct ByteTreeMeta {
    parameters: Vec<Box<dyn common::Meta>>,
}

impl ByteTreeMeta {
    pub fn new() -> Self {
        Self {
            parameters: vec![Box::new(
                crate::meta::wavelet_trees::layouts::common::BreadthFirstSearchMeta::new(),
            ) as Box<dyn common::Meta>],
        }
    }

    pub fn new_parameterized(parameters: Vec<Box<dyn common::Meta>>) -> Self {
        Self { parameters }
    }
}

impl common::Meta for ByteTreeMeta {
    fn file_specifications(
        &self,
        parameters_c_code: &Vec<String>,
        _parameters_file_specs: &Vec<Vec<common::FileSpecification>>,
        _id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        // Type does not have generic parameters. Do not specify unique ID.
        let header = get_header_specification(&parameters_c_code)?;
        Ok(vec![header])
    }
}

fn get_header_specification(parameters_c_code: &Vec<String>) -> Result<common::FileSpecification> {
    let file_name = std::path::PathBuf::from("byte_tree.hpp");

    Ok(common::FileSpecification {
        replacements: get_header_replacements(&parameters_c_code)?,
        template_file_name: file_name.clone(),
        target_file_name: file_name.clone(),
        c_file_type: common::CFileType::Hpp,
    })
}

fn get_header_replacements(
    parameters_c_code: &Vec<String>,
) -> Result<std::collections::BTreeMap<String, String>> {
    let value = parameters_c_code.first().ok_or(format_err!(
        "Failed to find any parameter values. Expected at least one."
    ))?;
    Ok(maplit::btreemap! {
        "#define LEX_ORDERED false".to_string() => format!("#define LEX_ORDERED {}", value)
    })
}

impl common::Path for ByteTreeMeta {
    fn path(&self) -> String {
        "sdsl::wavelet_trees::layouts::ByteTree".to_string()
    }
}

impl common::Code for ByteTreeMeta {
    fn c_code(&self, parameters_c_code: &Vec<String>) -> Result<String> {
        Ok(format!("sdsl::byte_tree<{}>", parameters_c_code.join(", ")))
    }
}

impl common::Parameters for ByteTreeMeta {
    fn parameters(&self) -> Vec<common::params::Parameter> {
        vec![common::params::Parameter::sdsl(0, true, 0)]
    }

    fn default_parameters_c_code(&self) -> Result<Vec<String>> {
        crate::interface::wavelet_trees::layouts::byte_tree::ByteTree::<
            crate::interface::wavelet_trees::layouts::common::BreadthFirstSearch,
        >::parameters_c_code()
    }

    fn parameters_meta(&self) -> &Vec<Box<dyn common::Meta>> {
        &self.parameters
    }
}
