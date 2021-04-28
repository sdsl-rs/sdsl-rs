use anyhow::{format_err, Result};

use crate::backend::common;
use crate::meta;

pub struct CodeMetadata {
    pub mir: String,
}

pub fn setup(
    crate_directory: &std::path::PathBuf,
    out_directory: &std::path::PathBuf,
) -> Result<Option<CodeMetadata>> {
    log::debug!("Generating code metadata.");
    Ok(match get_mir_file_path(&crate_directory, &out_directory)? {
        Some(path) => {
            let mir = std::fs::read_to_string(&path)?;
            Some(CodeMetadata { mir })
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
        .args(vec!["rustc", "--", "--emit=mir"])
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

pub fn analyse(code_metadata: &CodeMetadata) -> Result<Vec<CInterfaceSpec>> {
    log::debug!("Analyzing code metadata.");
    let mut interface_specs = Vec::<_>::new();

    for meta in meta::get_all()? {
        log::debug!("Identifying instances: {}", meta.path());

        if let Some(regex) = meta.default_regex()? {
            log::debug!("Identifying default generic parameters instances.");
            let capture_matches: Vec<_> = regex.captures_iter(&code_metadata.mir).collect();
            log::debug!("Found matches: {}", capture_matches.len());
            if !capture_matches.is_empty() {
                interface_specs.push(CInterfaceSpec::default(&meta)?);
            }
        }

        if let Some(regexes) = meta.parameters_regex()? {
            log::debug!("Identifying parameterized instances.");
            for regex in regexes {
                let capture_matches: Vec<_> = regex.captures_iter(&code_metadata.mir).collect();
                log::debug!("Found matches: {}", capture_matches.len());
                log::debug!("regex: {:?}", regex);

                for captures in capture_matches {
                    interface_specs.push(CInterfaceSpec::from_match_instance(&captures, &meta)?);
                }
            }
        }
    }
    Ok(interface_specs)
}

#[derive(Debug)]
pub struct CInterfaceSpec {
    pub id: String,
    pub replacements: std::collections::BTreeMap<String, String>,
    pub template_file_name: std::path::PathBuf,
    pub target_file_name: std::path::PathBuf,
}

impl CInterfaceSpec {
    pub fn default(meta: &Box<dyn meta::common::Meta>) -> Result<Self> {
        let id = get_id(None, &meta)?;
        let replacements = meta.replacements(None, &id);
        let template_file_name = meta.template_file_name();
        let target_file_name = get_target_file_name(&template_file_name, &id)?;
        Ok(Self {
            id,
            replacements,
            template_file_name: meta.template_file_name(),
            target_file_name: target_file_name,
        })
    }

    /// Construct from parameterized single match instance of meta struct in MIR.
    pub fn from_match_instance(
        captures: &regex::Captures,
        meta: &Box<dyn meta::common::Meta>,
    ) -> Result<Self> {
        log::debug!("parameters captures: {:?}", captures);
        let mut parameters_values = Vec::<_>::new();
        for (index, _parameter) in meta.parameters().iter().enumerate() {
            // +1 because skipping index 0 which contains the whole match
            let value = captures.get(index + 1).map_or("", |m| m.as_str());
            parameters_values.push(value.to_string());
        }

        let id = get_id(Some(&parameters_values), &meta)?;
        let replacements = meta.replacements(Some(&parameters_values), &id);
        let template_file_name = meta.template_file_name();
        let target_file_name = get_target_file_name(&template_file_name, &id)?;

        Ok(Self {
            id,
            replacements,
            template_file_name: meta.template_file_name(),
            target_file_name: target_file_name,
        })
    }
}

fn get_id(
    parameters_values: Option<&Vec<String>>,
    meta: &Box<dyn meta::common::Meta>,
) -> Result<String> {
    let mut hasher = blake3::Hasher::new();

    add_serialized(&parameters_values, &mut hasher)?;
    add_serialized(&meta.template_file_name(), &mut hasher)?;

    let hash = hasher.finalize().to_hex().as_str().to_string();
    Ok(hash.chars().take(32).collect())
}

fn add_serialized<T: serde::Serialize>(input: &T, hasher: &mut blake3::Hasher) -> Result<()> {
    let serialized = bincode::serialize(&input)?;
    let serialized = serialized.as_slice();
    hasher.update(serialized);
    Ok(())
}

fn get_target_file_name(
    template_file_name: &std::path::PathBuf,
    id: &str,
) -> Result<std::path::PathBuf> {
    let stem = template_file_name
        .file_stem()
        .and_then(|s| s.to_str().to_owned())
        .ok_or(format_err!(
            "Failed to find stem for file: {}",
            template_file_name.display()
        ))?;
    let extension = template_file_name
        .extension()
        .and_then(|s| s.to_str().to_owned())
        .ok_or(format_err!(
            "Failed to find extension for file: {}",
            template_file_name.display()
        ))?;
    let target_file_name = format!(
        "{stem}_{id}.{extension}",
        stem = stem,
        id = id,
        extension = extension
    );
    Ok(std::path::PathBuf::from(target_file_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() -> Result<()> {
        let code_metadata = CodeMetadata {
            mir: "sdsl::int_vector::IntVector<1_u32>;".to_string(),
        };
        let interface_specs = analyse(&code_metadata)?;
        println!("{:?}", interface_specs);
        Ok(())
    }
}
