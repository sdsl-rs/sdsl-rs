use anyhow::{format_err, Result};

use crate::backend::{common, sdsl_c::specification};
use crate::meta;

pub struct CodeMeta {
    pub mir: String,
}

pub fn setup(
    crate_directory: &std::path::PathBuf,
    out_directory: &std::path::PathBuf,
) -> Result<Option<CodeMeta>> {
    log::debug!("Generating code metadata.");
    Ok(match get_mir_file_path(&crate_directory, &out_directory)? {
        Some(path) => {
            let mir = std::fs::read_to_string(&path)?;
            Some(CodeMeta { mir })
        }
        None => None,
    })
}

fn get_mir_file_path(
    crate_directory: &std::path::PathBuf,
    out_directory: &std::path::PathBuf,
) -> Result<Option<std::path::PathBuf>> {
    log::debug!("Constructing MIR file.");
    let mir_tmp_directory = out_directory.join("mir_build");
    std::fs::create_dir_all(&mir_tmp_directory)?;

    let mut child = std::process::Command::new("cargo")
        .args(vec!["rustc", "--tests", "--", "--emit=mir"])
        .env(common::ENV_SKIP_BUILD, "1")
        .env("CARGO_TARGET_DIR", &mir_tmp_directory)
        .current_dir(crate_directory)
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to generate MIR file.");
    let exit_status = child.wait()?;
    if !exit_status.success() {
        log::debug!("Cargo build step failed. MIR file not generated.");
        return Ok(None);
    }

    let deps_directory = mir_tmp_directory.join("debug/deps");
    let entry = walkdir::WalkDir::new(&deps_directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("mir"))
        .next();
    let entry = entry.ok_or(format_err!("Failed to find MIR file."))?;

    let mir_file_path = out_directory.join("mir");
    std::fs::copy(entry.path(), &mir_file_path)?;

    log::debug!("Found MIR file: {}", mir_file_path.display());
    Ok(Some(mir_file_path))
}

pub fn analyse(code_meta: &CodeMeta) -> Result<Vec<specification::Specification>> {
    log::debug!("Analyzing code metadata.");
    let mut interface_specs = Vec::<_>::new();

    for meta in meta::get_all()? {
        log::debug!("Identifying instances: {}", meta.path());

        if let Some(spec) = default_specification(&code_meta, &meta)? {
            interface_specs.push(spec);
        }

        let specs = parameterized_specifications(&code_meta, &meta)?;
        interface_specs.extend(specs);
    }
    Ok(interface_specs)
}

fn default_specification(
    code_meta: &CodeMeta,
    meta: &Box<dyn meta::common::Meta>,
) -> Result<Option<specification::Specification>> {
    if let Some(regex) = meta.default_regex()? {
        let capture_matches: Vec<_> = regex.captures_iter(&code_meta.mir).collect();
        if !capture_matches.is_empty() {
            return Ok(Some(specification::Specification::new(
                &Vec::<_>::new(),
                &meta,
            )?));
        }
    }
    Ok(None)
}

fn parameterized_specifications(
    code_meta: &CodeMeta,
    meta: &Box<dyn meta::common::Meta>,
) -> Result<Vec<specification::Specification>> {
    let mut specs = Vec::<_>::new();

    let regexes = match meta.parameters_regex()? {
        Some(regexes) => regexes,
        None => return Ok(specs),
    };

    for regex in regexes {
        let capture_matches: Vec<_> = regex.captures_iter(&code_meta.mir).collect();
        for captures in capture_matches {
            let parameter_values = parameter_values(&captures, &meta)?;
            let spec = specification::Specification::new(&parameter_values, &meta)?;
            specs.push(spec);
        }
    }

    Ok(specs)
}

fn parameter_values(
    captures: &regex::Captures,
    meta: &Box<dyn meta::common::Meta>,
) -> Result<Vec<String>> {
    let mut values = Vec::<_>::new();
    for (index, parameter) in meta.parameters().iter().enumerate() {
        let capture_group_name = meta::common::params::get_capture_group_name(index);
        let mut value = captures
            .name(&capture_group_name)
            .map_or("", |m| m.as_str())
            .to_string();
        if parameter.is_sdsl_type {
            let spec = handle_sdsl_type(&value)?;
            value = spec.c_code.clone();
        }

        values.push(value);
    }
    Ok(values)
}

fn handle_sdsl_type(parameter_value: &str) -> Result<specification::Specification> {
    let prefix = " ";
    let specification = analyse(&CodeMeta {
        mir: format!(
            "{prefix}{parameter};",
            prefix = prefix,
            parameter = parameter_value.to_string()
        ),
    })?
    .into_iter()
    .next()
    .ok_or(format_err!(
        "Expected single SDSL match for parameter value: {}",
        parameter_value
    ))?;
    Ok(specification)
}

#[test]
fn test_rank() -> Result<()> {
    let mir = "    let mut _43: &sdsl::RankSupportV<sdsl::bit_patterns::P01>; // in scope 0 at examples/src/rank_support_v.rs:8:18: 8:20";
    let x = analyse(&CodeMeta {
        mir: mir.to_string(),
    })?;
    println!("{:?}", x);
    Ok(())
}
