use anyhow::{format_err, Result};
use sharedlib::Symbol;

pub mod specification;
pub mod template;

// Ensure shared lib is loaded once. Ensure lib does not fail to load due to IO race conditions.
lazy_static::lazy_static! {
    pub static ref LIB: std::sync::Arc<sharedlib::Lib> = get_lib().unwrap();
}

pub fn get_lib() -> Result<std::sync::Arc<sharedlib::Lib>> {
    let out_directory = std::env::var("OUT_DIR").map_err(|e| format_err!("{}", e))?;
    let out_directory = std::path::PathBuf::from(&out_directory);
    let lib_path = out_directory.join("lib").join("libsdsl_c.so");
    unsafe {
        let lib = sharedlib::Lib::new(lib_path).map_err(|e| format_err!("{}", e))?;
        Ok(std::sync::Arc::new(lib))
    }
}

pub struct FunctionBuilder {
    base_name: Option<String>,
    id: String,
    lib: std::sync::Arc<sharedlib::Lib>,
}

impl FunctionBuilder {
    pub fn new<'a>(base_name: Option<&str>, id: &str, lib: std::sync::Arc<sharedlib::Lib>) -> Self {
        Self {
            base_name: base_name.and_then(|f| Some(f.to_string())),
            id: id.to_string(),
            lib,
        }
    }

    pub fn get<'a, T: Copy>(&self, name: &str) -> Result<T> {
        let full_name = self.full_name(&name);
        unsafe {
            let symbol: sharedlib::Func<T> = self
                .lib
                .find_func(full_name)
                .map_err(|e| format_err!("{}", e))?;
            let function = symbol.get();
            Ok(function)
        }
    }

    fn full_name(&self, name: &str) -> String {
        match &self.base_name {
            Some(base_name) => format!(
                "{base_name}_{name}_{id}",
                base_name = base_name,
                name = name,
                id = self.id
            ),
            None => format!("{name}_{id}", name = name, id = self.id),
        }
    }
}
