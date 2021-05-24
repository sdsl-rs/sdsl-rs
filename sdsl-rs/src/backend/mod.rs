use anyhow::{format_err, Result};

mod analyse;
mod common;
pub mod sdsl_c;

/// Build the SDSL interface backend.
///
/// This function should be executed in the project's [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
/// It analyses the project's code base and builds a suitable SDSL interface backend.
///
/// # Example
/// ```ignore
/// // build.rs
/// fn main() {
///     match sdsl::build() {
///         Ok(_) => {}
///         Err(e) => panic!("Error: {}", e),
///     };
/// }
/// ```
/// A working example can be found [here](https://github.com/sdsl-rs/sdsl-rs/blob/master/examples/build.rs).
pub fn build() -> Result<()> {
    simple_logger::SimpleLogger::new().init()?;

    if common::skip_build() {
        return Ok(());
    }

    let out_directory = std::env::var("OUT_DIR").map_err(|e| format_err!("{}", e))?;
    let out_directory = std::path::PathBuf::from(&out_directory);

    let crate_directory = get_crate_directory()?;
    let code_meta = analyse::setup(&crate_directory, &out_directory)?;

    let specifications = if let Some(code_meta) = code_meta {
        analyse::analyse(&code_meta)?
    } else {
        log::debug!("Failed to generate code metadata for analysis. Exiting SDSL build.");
        return Ok(());
    };

    let template_directory = sdsl_c::template::setup(&out_directory)?;
    let interface_directory =
        sdsl_c::specification::setup(&specifications, &template_directory, &out_directory)?;
    let lib_path = sdsl_c::specification::compile(&interface_directory)?;
    log::info!("Compilation complete. Library path: {}", lib_path.display());

    println!("cargo:rerun-if-changed=./src");
    Ok(())
}

fn get_crate_directory() -> Result<std::path::PathBuf> {
    let cargo_manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").ok_or(format_err!(
        "Failed to read env variable: CARGO_MANIFEST_DIR"
    ))?;
    Ok(std::path::PathBuf::from(&cargo_manifest_dir))
}
