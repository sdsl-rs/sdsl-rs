use crate::meta::common;
use anyhow::Result;

// TODO: Depth first search.

pub struct BreadthFirstSearchMeta {
    parameters: Vec<Box<dyn common::Meta>>,
}

impl BreadthFirstSearchMeta {
    pub fn new() -> Self {
        Self { parameters: vec![] }
    }
}

impl common::Meta for BreadthFirstSearchMeta {
    fn file_specifications(
        &self,
        _parameters_c_code: &Vec<String>,
        _parameters_file_specs: &Vec<Vec<common::FileSpecification>>,
        _id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        Ok(vec![])
    }
}

impl common::Path for BreadthFirstSearchMeta {
    fn path(&self) -> String {
        "sdsl::wavelet_trees::layouts::BreadthFirstSearch".to_string()
    }
}

impl common::Code for BreadthFirstSearchMeta {
    fn c_code(&self, _parameters_c_code: &Vec<String>) -> Result<String> {
        Ok("false".to_string())
    }
}

impl common::Parameters for BreadthFirstSearchMeta {
    fn parameters(&self) -> Vec<common::params::Parameter> {
        vec![]
    }

    fn default_parameters_c_code(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn parameters_meta(&self) -> &Vec<Box<dyn common::Meta>> {
        &self.parameters
    }
}
