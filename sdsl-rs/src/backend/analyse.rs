use anyhow::{format_err, Result};

use crate::backend::{common, sdsl_c::specification};
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

pub fn analyse(code_metadata: &CodeMetadata) -> Result<Vec<specification::Specification>> {
    log::debug!("Analyzing code metadata.");
    let mut interface_specs = Vec::<_>::new();

    for meta in meta::get_all()? {
        log::debug!("Identifying instances: {}", meta.path());

        if let Some(regex) = meta.default_regex()? {
            let capture_matches: Vec<_> = regex.captures_iter(&code_metadata.mir).collect();
            if !capture_matches.is_empty() {
                interface_specs.push(specification::Specification::default(&meta)?);
            }
        }

        if let Some(regexes) = meta.parameters_regex()? {
            for regex in regexes {
                let capture_matches: Vec<_> = regex.captures_iter(&code_metadata.mir).collect();
                for captures in capture_matches {
                    interface_specs.push(specification::Specification::from_match_instance(
                        &captures, &meta,
                    )?);
                }
            }
        }
    }
    Ok(interface_specs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta;

    #[test]
    fn test_int_vector_correct_specifications() -> Result<()> {
        let code_metadata = CodeMetadata {
            mir: "sdsl::int_vector::IntVector<1_u32>;".to_string(),
        };
        let result = analyse(&code_metadata)?;

        let expected = vec![specification::Specification {
            id: "4ca7a94e593428ab3e8c2cd3e6b936e3".to_string(),
            files: vec![
                meta::common::FileSpecification {
                    replacements: std::collections::BTreeMap::<String, String>::new(),
                    template_file_name: std::path::PathBuf::from("int_vector.cpp"),
                    target_file_name: std::path::PathBuf::from(
                        "int_vector_4ca7a94e593428ab3e8c2cd3e6b936e3.cpp",
                    ),
                    c_file_type: meta::common::CFileType::Cpp,
                },
                meta::common::FileSpecification {
                    replacements: maplit::btreemap! {
                        "#define WT_INT_ID".to_string() => "#define WT_INT_ID _4ca7a94e593428ab3e8c2cd3e6b936e3".to_string(),
                        "#define WT_INT_TEMPLATE".to_string() => "#define WT_INT_TEMPLATE 1".to_string(),
                    },
                    template_file_name: std::path::PathBuf::from("int_vector.hpp"),
                    target_file_name: std::path::PathBuf::from(
                        "int_vector_4ca7a94e593428ab3e8c2cd3e6b936e3.hpp",
                    ),
                    c_file_type: meta::common::CFileType::Hpp,
                },
            ],
        }];
        assert_eq!(result, expected);
        Ok(())
    }
}
