use crate::meta::common;
use anyhow::Result;

pub struct P0Meta;

impl P0Meta {
    pub fn new() -> Self {
        Self {}
    }
}

impl common::Meta for P0Meta {
    fn file_specifications(
        &self,
        _parameter_values: &Vec<String>,
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
    fn c_code(&self, _parameter_values: &Vec<String>) -> Result<String> {
        Ok("0, 1".to_string())
    }
}

impl common::Parameters for P0Meta {
    fn parameters(&self) -> Vec<common::params::Parameter> {
        vec![]
    }
}

pub struct P1Meta;

impl P1Meta {
    pub fn new() -> Self {
        Self {}
    }
}

impl common::Meta for P1Meta {
    fn file_specifications(
        &self,
        _parameter_values: &Vec<String>,
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
    fn c_code(&self, _parameter_values: &Vec<String>) -> Result<String> {
        Ok("1, 1".to_string())
    }
}

impl common::Parameters for P1Meta {
    fn parameters(&self) -> Vec<common::params::Parameter> {
        vec![]
    }
}

pub struct P10Meta;

impl P10Meta {
    pub fn new() -> Self {
        Self {}
    }
}

impl common::Meta for P10Meta {
    fn file_specifications(
        &self,
        _parameter_values: &Vec<String>,
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
    fn c_code(&self, _parameter_values: &Vec<String>) -> Result<String> {
        Ok("10, 2".to_string())
    }
}

impl common::Parameters for P10Meta {
    fn parameters(&self) -> Vec<common::params::Parameter> {
        vec![]
    }
}

pub struct P01Meta;

impl P01Meta {
    pub fn new() -> Self {
        Self {}
    }
}

impl common::Meta for P01Meta {
    fn file_specifications(
        &self,
        _parameter_values: &Vec<String>,
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
    fn c_code(&self, _parameter_values: &Vec<String>) -> Result<String> {
        Ok("1, 2".to_string())
    }
}

impl common::Parameters for P01Meta {
    fn parameters(&self) -> Vec<common::params::Parameter> {
        vec![]
    }
}
