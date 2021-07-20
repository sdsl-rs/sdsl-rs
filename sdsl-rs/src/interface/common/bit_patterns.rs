use crate::meta;
use anyhow::Result;

pub trait BitPattern: std::fmt::Display + super::Code {
    /// Returns bit pattern and bit pattern length.
    fn bit_pattern(&self) -> (u8, u8);
    fn to_string() -> String;
}

#[derive(Debug)]
pub struct P0;

impl std::fmt::Display for P0 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("P0").finish()
    }
}

impl BitPattern for P0 {
    fn bit_pattern(&self) -> (u8, u8) {
        (0, 1)
    }
    fn to_string() -> String {
        "P0".to_string()
    }
}

impl super::Code for P0 {
    fn c_code() -> Result<String> {
        let meta =
            Box::new(meta::common::bit_patterns::P0Meta::new()) as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        Ok(meta.c_code(&parameters_c_code)?)
    }

    fn parameters_c_code() -> Result<Vec<String>> {
        Ok(vec![])
    }
}

pub struct P1;

impl std::fmt::Display for P1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("P1").finish()
    }
}

impl BitPattern for P1 {
    fn bit_pattern(&self) -> (u8, u8) {
        (1, 1)
    }
    fn to_string() -> String {
        "P1".to_string()
    }
}

impl super::Code for P1 {
    fn c_code() -> Result<String> {
        let meta =
            Box::new(meta::common::bit_patterns::P1Meta::new()) as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        Ok(meta.c_code(&parameters_c_code)?)
    }

    fn parameters_c_code() -> Result<Vec<String>> {
        Ok(vec![])
    }
}

pub struct P10;

impl std::fmt::Display for P10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("P10").finish()
    }
}

impl BitPattern for P10 {
    fn bit_pattern(&self) -> (u8, u8) {
        (10, 2)
    }
    fn to_string() -> String {
        "P10".to_string()
    }
}

impl super::Code for P10 {
    fn c_code() -> Result<String> {
        let meta =
            Box::new(meta::common::bit_patterns::P10Meta::new()) as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        Ok(meta.c_code(&parameters_c_code)?)
    }

    fn parameters_c_code() -> Result<Vec<String>> {
        Ok(vec![])
    }
}

#[derive(Debug)]
pub struct P01;

impl std::fmt::Display for P01 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("P01").finish()
    }
}

impl BitPattern for P01 {
    fn bit_pattern(&self) -> (u8, u8) {
        (1, 2)
    }
    fn to_string() -> String {
        "P01".to_string()
    }
}

impl super::Code for P01 {
    fn c_code() -> Result<String> {
        let meta =
            Box::new(meta::common::bit_patterns::P01Meta::new()) as Box<dyn meta::common::Meta>;
        let parameters_c_code = Self::parameters_c_code()?;
        Ok(meta.c_code(&parameters_c_code)?)
    }

    fn parameters_c_code() -> Result<Vec<String>> {
        Ok(vec![])
    }
}
