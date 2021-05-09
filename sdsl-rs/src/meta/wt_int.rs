use crate::meta::common;

pub struct WtIntMeta;

impl WtIntMeta {
    pub fn new() -> Self {
        Self {}
    }
}

impl common::Meta for WtIntMeta {
    fn template_file_names(&self) -> common::TemplateFileNames {
        common::TemplateFileNames {
            source: std::path::PathBuf::from("wt_int.cpp"),
            header: std::path::PathBuf::from("wt_int.hpp"),
        }
    }

    fn replacements(
        &self,
        _parameter_values: Option<&Vec<String>>,
        _id: &str,
    ) -> std::collections::BTreeMap<String, String> {
        maplit::btreemap! {}
    }
}

impl common::Path for WtIntMeta {
    fn path(&self) -> String {
        "sdsl::wt_int::WtInt".to_string()
    }
}

impl common::params::Parameters for WtIntMeta {
    fn parameters(&self) -> Vec<common::params::Parameter> {
        vec![
            common::params::Parameter::sdsl(0, true),
            common::params::Parameter::sdsl(1, true),
            common::params::Parameter::sdsl(2, true),
            common::params::Parameter::sdsl(4, true),
        ]
    }
}

struct Interface {
    get: extern "C" fn(*mut libc::c_void, u64) -> usize,
}

impl Interface {
    pub fn new(id: &String, lib: &sharedlib::Lib) -> Result<Self> {
        Ok(Self {
            get: sdsl_c::get_function("int_vector_get_element", Some(&id), &lib)?,
        })
    }
}