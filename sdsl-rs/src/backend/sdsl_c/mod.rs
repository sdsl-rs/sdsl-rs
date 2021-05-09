use anyhow::{format_err, Result};
use sharedlib::Symbol;

pub mod specification;
pub mod template;

pub fn get_lib() -> Result<sharedlib::Lib> {
    let out_directory = std::env::var("OUT_DIR").map_err(|e| format_err!("{}", e))?;
    let out_directory = std::path::PathBuf::from(&out_directory);
    let lib_path = out_directory.join("lib").join("libsdsl_c.so");
    unsafe { Ok(sharedlib::Lib::new(lib_path).map_err(|e| format_err!("{}", e))?) }
}

pub fn get_function<'a, T: Copy>(name: &str, id: Option<&str>, lib: &'a sharedlib::Lib) -> Result<T> {
    let name = match id {
        Some(id) => format!("{name}_{id}", name=name, id=id),
        None => name.to_string()
    };
    unsafe {
        let symbol: sharedlib::Func<T> =
            lib.find_func(name).map_err(|e| format_err!("{}", e))?;
        let function = symbol.get();
        Ok(function)
    }
}
// pub fn get_function<'a>(name: &str, id: Option<&str>, lib: &'a sharedlib::Lib) -> Result<extern "C" fn()> {
//     let name = match id {
//         Some(id) => format!("{name}_{id}", name=name, id=id),
//         None => name.to_string()
//     };
//     unsafe {
//         let symbol: sharedlib::Func<extern "C" fn()> =
//             lib.find_func(name).map_err(|e| format_err!("{}", e))?;
//         let function = symbol.get();
//         Ok(function)
//     }
// }