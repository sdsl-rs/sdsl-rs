use crate::meta::common;
use anyhow::Result;

pub struct P0Meta {
    parameters_default_meta: Vec<Box<dyn common::Meta>>,
}

impl P0Meta {
    pub fn new() -> Self {
        Self {
            parameters_default_meta: vec![],
        }
    }
}

impl common::Meta for P0Meta {
    fn file_specifications(
        &self,
        _parameters_c_code: &Vec<String>,
        _parameters_file_specs: &Vec<Vec<common::FileSpecification>>,
        _id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        Ok(vec![])
    }
}

impl common::Path for P0Meta {
    fn path(&self) -> String {
        "sdsl::bit_patterns::P0".to_string()
    }
}

impl common::Code for P0Meta {
    fn c_code(&self, _parameters_c_code: &Vec<String>) -> Result<String> {
        Ok("0, 1".to_string())
    }
}

impl common::Parameters for P0Meta {
    fn parameters_definitions(&self) -> Vec<common::params::Parameter> {
        vec![]
    }

    fn parameters_default_c_code(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn parameters_default_meta(&self) -> &Vec<Box<dyn common::Meta>> {
        &self.parameters_default_meta
    }
}

pub struct P1Meta {
    parameters_default_meta: Vec<Box<dyn common::Meta>>,
}

impl P1Meta {
    pub fn new() -> Self {
        Self {
            parameters_default_meta: vec![],
        }
    }
}

impl common::Meta for P1Meta {
    fn file_specifications(
        &self,
        _parameters_c_code: &Vec<String>,
        _parameters_file_specs: &Vec<Vec<common::FileSpecification>>,
        _id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        Ok(vec![])
    }
}

impl common::Path for P1Meta {
    fn path(&self) -> String {
        "sdsl::bit_patterns::P1".to_string()
    }
}

impl common::Code for P1Meta {
    fn c_code(&self, _parameters_c_code: &Vec<String>) -> Result<String> {
        Ok("1, 1".to_string())
    }
}

impl common::Parameters for P1Meta {
    fn parameters_definitions(&self) -> Vec<common::params::Parameter> {
        vec![]
    }

    fn parameters_default_c_code(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn parameters_default_meta(&self) -> &Vec<Box<dyn common::Meta>> {
        &self.parameters_default_meta
    }
}

pub struct P10Meta {
    parameters_default_meta: Vec<Box<dyn common::Meta>>,
}

impl P10Meta {
    pub fn new() -> Self {
        Self {
            parameters_default_meta: vec![],
        }
    }
}

impl common::Meta for P10Meta {
    fn file_specifications(
        &self,
        _parameters_c_code: &Vec<String>,
        _parameters_file_specs: &Vec<Vec<common::FileSpecification>>,
        _id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        Ok(vec![])
    }
}

impl common::Path for P10Meta {
    fn path(&self) -> String {
        "sdsl::bit_patterns::P10".to_string()
    }
}

impl common::Code for P10Meta {
    fn c_code(&self, _parameters_c_code: &Vec<String>) -> Result<String> {
        Ok("10, 2".to_string())
    }
}

impl common::Parameters for P10Meta {
    fn parameters_definitions(&self) -> Vec<common::params::Parameter> {
        vec![]
    }

    fn parameters_default_c_code(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn parameters_default_meta(&self) -> &Vec<Box<dyn common::Meta>> {
        &self.parameters_default_meta
    }
}

pub struct P01Meta {
    parameters_default_meta: Vec<Box<dyn common::Meta>>,
}

impl P01Meta {
    pub fn new() -> Self {
        Self {
            parameters_default_meta: vec![],
        }
    }
}

impl common::Meta for P01Meta {
    fn file_specifications(
        &self,
        _parameters_c_code: &Vec<String>,
        _parameters_file_specs: &Vec<Vec<common::FileSpecification>>,
        _id: &str,
    ) -> Result<Vec<common::FileSpecification>> {
        Ok(vec![])
    }
}

impl common::Path for P01Meta {
    fn path(&self) -> String {
        "sdsl::bit_patterns::P01".to_string()
    }
}

impl common::Code for P01Meta {
    fn c_code(&self, _parameters_c_code: &Vec<String>) -> Result<String> {
        Ok("1, 2".to_string())
    }
}

impl common::Parameters for P01Meta {
    fn parameters_definitions(&self) -> Vec<common::params::Parameter> {
        vec![]
    }

    fn parameters_default_c_code(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn parameters_default_meta(&self) -> &Vec<Box<dyn common::Meta>> {
        &self.parameters_default_meta
    }
}
