use crate::meta;
use anyhow::{format_err, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Specification {
    pub id: String,
    pub replacements: std::collections::BTreeMap<String, String>,
    pub template_file_name: std::path::PathBuf,
    pub target_file_name: std::path::PathBuf,
}

impl Specification {
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

pub fn setup(
    specifications: &Vec<Specification>,
    template_directory: &std::path::PathBuf,
    out_directory: &std::path::PathBuf,
) -> Result<std::path::PathBuf> {
    log::debug!("Setting up template specification directory.");
    let (interface_directory, include_directory) =
        setup_static_files(&out_directory, &template_directory)?;
    setup_include_files(&specifications, &template_directory, &include_directory)?;
    Ok(interface_directory)
}

fn setup_include_files(
    specifications: &Vec<Specification>,
    template_directory: &std::path::PathBuf,
    include_directory: &std::path::PathBuf,
) -> Result<()> {
    log::debug!("Setting up include files.");

    let mut added_include_files = std::collections::BTreeSet::<_>::new();
    for specification in specifications {
        let template_file_path = template_directory
            .join("include")
            .join(&specification.template_file_name);
        let target_file_path = include_directory.join(&specification.target_file_name);
        added_include_files.insert(target_file_path.clone());

        if !target_file_path.exists() {
            std::fs::copy(&template_file_path, &target_file_path)?;
        }
    }
    cleanup_stale_include_files(&added_include_files, &include_directory)?;
    Ok(())
}

fn cleanup_stale_include_files(
    files_to_keep: &std::collections::BTreeSet<std::path::PathBuf>,
    include_directory: &std::path::PathBuf,
) -> Result<()> {
    for path in walkdir::WalkDir::new(&include_directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|entry| entry.path().to_path_buf())
    {
        if !files_to_keep.contains(&path) {
            log::debug!("Removing stale include file: {}", path.display());
            std::fs::remove_file(&path)?;
        }
    }
    Ok(())
}

fn setup_static_files(
    out_directory: &std::path::PathBuf,
    template_directory: &std::path::PathBuf,
) -> Result<(std::path::PathBuf, std::path::PathBuf)> {
    log::debug!("Setting up static files.");

    let interface_directory = out_directory.join("sdsl-c");
    if !interface_directory.exists() {
        std::fs::create_dir_all(&interface_directory)?;
    }

    let include_directory = interface_directory.join("include");
    if !include_directory.exists() {
        std::fs::create_dir_all(&include_directory)?;
    }

    let template_lib_directory = template_directory.join("lib");
    if !interface_directory.join("lib").exists() {
        fs_extra::copy_items(
            &vec![template_lib_directory],
            &interface_directory,
            &fs_extra::dir::CopyOptions::new(),
        )?;
    }

    let cmakelists_file_path = interface_directory.join("CMakeLists.txt");
    if !cmakelists_file_path.exists() {
        let template_cmakelists_file_path = template_directory
            .join("miscellaneous")
            .join("template_CMakeLists.txt");
        std::fs::copy(&template_cmakelists_file_path, &cmakelists_file_path)?;
    }
    Ok((interface_directory, include_directory))
}
