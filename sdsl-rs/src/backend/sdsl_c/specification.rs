use crate::meta;
use anyhow::{format_err, Result};
use std::io::Write;

/// A specification of a meta type and its parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Specification {
    pub files: Vec<meta::common::FileSpecification>,
    pub c_code: String,
}

impl Specification {
    /// Create a specification from a meta and its parameter specifications.
    pub fn from_parameterized_meta(
        parameters_specifications: &Vec<Specification>,
        meta: &Box<dyn meta::common::Meta>,
    ) -> Result<Self> {
        let parameters_c_code = parameters_specifications
            .iter()
            .map(|spec| spec.c_code.clone())
            .collect();
        let parameters_file_specs: Vec<Vec<meta::common::FileSpecification>> =
            parameters_specifications
                .iter()
                .map(|spec| spec.files.clone())
                .collect();
        let id = get_id(&meta.c_code(&parameters_c_code)?)?;
        Ok(Self {
            files: meta.file_specifications(&parameters_c_code, &parameters_file_specs, &id)?,
            c_code: meta.c_code(&parameters_c_code)?,
        })
    }

    /// Create a specification from a meta and its default parameter values.
    pub fn from_default_meta(meta: &Box<dyn meta::common::Meta>) -> Result<Self> {
        let parameters_c_code = meta.default_parameters_c_code()?;
        let mut parameters_file_specs = vec![];
        for parameter_meta in meta.parameters_meta() {
            parameters_file_specs.push(Self::from_default_meta(&parameter_meta)?.files);
        }
        let id = get_id(&meta.c_code(&parameters_c_code)?)?;

        Ok(Self {
            files: meta.file_specifications(&parameters_c_code, &parameters_file_specs, &id)?,
            c_code: meta.c_code(&parameters_c_code)?,
        })
    }
}

pub fn get_id(c_code: &str) -> Result<String> {
    let mut hasher = blake3::Hasher::new();
    add_serialized(&c_code, &mut hasher)?;
    let hash = hasher.finalize().to_hex().as_str().to_string();
    Ok(hash.chars().take(32).collect())
}

fn add_serialized<T: serde::Serialize>(input: &T, hasher: &mut blake3::Hasher) -> Result<()> {
    let serialized = bincode::serialize(&input)?;
    let serialized = serialized.as_slice();
    hasher.update(serialized);
    Ok(())
}

pub fn setup(
    specifications: &Vec<Specification>,
    template_directory: &std::path::PathBuf,
    out_directory: &std::path::PathBuf,
) -> Result<std::path::PathBuf> {
    log::debug!("Setting up template specification directory.");
    let (interface_directory, src_directory, include_directory) =
        setup_static_files(&out_directory, &template_directory)?;
    let new_files = setup_source_files(
        &specifications,
        &template_directory,
        &src_directory,
        &include_directory,
    )?;
    ensure_replacements(
        &specifications,
        &src_directory,
        &include_directory,
        &new_files,
    )?;
    Ok(interface_directory)
}

fn setup_source_files(
    specifications: &Vec<Specification>,
    template_directory: &std::path::PathBuf,
    src_directory: &std::path::PathBuf,
    include_directory: &std::path::PathBuf,
) -> Result<std::collections::BTreeSet<std::path::PathBuf>> {
    log::debug!("Setting up source files.");

    let mut new_files = std::collections::BTreeSet::<_>::new();
    let mut added_files = std::collections::BTreeSet::<_>::new();
    for specification in specifications {
        for file_specification in &specification.files {
            let (template_file_path, target_file_path) = match file_specification.c_file_type {
                meta::common::CFileType::Hpp => {
                    let template_file_path = template_directory
                        .join("include")
                        .join(&file_specification.template_file_name);
                    let target_file_path =
                        include_directory.join(&file_specification.target_file_name);
                    (template_file_path, target_file_path)
                }
                meta::common::CFileType::Cpp => {
                    let template_file_path = template_directory
                        .join("src")
                        .join(&file_specification.template_file_name);
                    let target_file_path = src_directory.join(&file_specification.target_file_name);
                    (template_file_path, target_file_path)
                }
            };

            added_files.insert(target_file_path.clone());
            if !target_file_path.exists() {
                std::fs::copy(&template_file_path, &target_file_path)?;
                new_files.insert(target_file_path.clone());
            }
        }
    }
    cleanup_stale_files(&added_files, &include_directory)?;
    cleanup_stale_files(&added_files, &src_directory)?;
    Ok(new_files)
}

fn cleanup_stale_files(
    files_to_keep: &std::collections::BTreeSet<std::path::PathBuf>,
    directory: &std::path::PathBuf,
) -> Result<()> {
    for path in walkdir::WalkDir::new(&directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|entry| entry.path().to_path_buf())
    {
        let file_name = path
            .file_name()
            .and_then(|f| f.to_str())
            .ok_or(format_err!("Failed to obtain file name."))?;

        if !files_to_keep.contains(&path) && file_name != "common.hpp" {
            log::debug!("Removing stale file: {}", path.display());
            std::fs::remove_file(&path)?;
        }
    }
    Ok(())
}

fn ensure_replacements(
    specifications: &Vec<Specification>,
    src_directory: &std::path::PathBuf,
    include_directory: &std::path::PathBuf,
    new_files: &std::collections::BTreeSet<std::path::PathBuf>,
) -> Result<()> {
    log::debug!("Editing source files with specification replacements.");

    let mut processed_files = std::collections::BTreeSet::<std::path::PathBuf>::new();
    for specification in specifications {
        for file_specification in &specification.files {
            let source_directory = match file_specification.c_file_type {
                meta::common::CFileType::Hpp => include_directory,
                meta::common::CFileType::Cpp => src_directory,
            };

            let target_file_path = source_directory.join(&file_specification.target_file_name);

            // Ensure files are only processed once.
            if !new_files.contains(&target_file_path) || processed_files.contains(&target_file_path)
            {
                continue;
            }

            replace_lines(&target_file_path, &file_specification.replacements)?;
            processed_files.insert(target_file_path.clone());
        }
    }
    Ok(())
}

fn replace_lines(
    path: &std::path::PathBuf,
    replacements: &std::collections::BTreeMap<String, String>,
) -> Result<()> {
    log::debug!("Actualize line replacement in file: {}", path.display());

    let content = std::fs::read_to_string(&path)?;
    let content_split: Vec<&str> = content.split('\n').collect();

    let mut new_contents = Vec::<String>::new();
    for line in content_split {
        let mut line = line.to_string();
        for (from, to) in replacements {
            line = line.replace(&from.as_str(), &to.as_str());
        }
        new_contents.push(line);
    }

    std::fs::remove_file(&path)?;

    let file = std::fs::File::create(&path).expect("Failed to create file");
    let mut buffered_out = std::io::BufWriter::new(&file);
    buffered_out.write_all(new_contents.join("\n").as_bytes())?;
    file.sync_all()?;

    Ok(())
}

fn setup_static_files(
    out_directory: &std::path::PathBuf,
    template_directory: &std::path::PathBuf,
) -> Result<(std::path::PathBuf, std::path::PathBuf, std::path::PathBuf)> {
    log::debug!("Setting up static files.");

    let interface_directory = out_directory.join("sdsl-c");
    if !interface_directory.exists() {
        std::fs::create_dir_all(&interface_directory)?;
    }

    let include_directory = interface_directory.join("include");
    if !include_directory.exists() {
        std::fs::create_dir_all(&include_directory)?;
    }
    let common_include_file = interface_directory.join("include").join("common.hpp");
    if !common_include_file.exists() {
        std::fs::copy(
            &template_directory.join("include").join("common.hpp"),
            &common_include_file,
        )?;
    }

    let src_directory = interface_directory.join("src");
    if !src_directory.exists() {
        std::fs::create_dir_all(&src_directory)?;
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
    Ok((interface_directory, src_directory, include_directory))
}

pub fn compile(interface_directory: &std::path::PathBuf) -> Result<std::path::PathBuf> {
    log::info!("Compiling SDSL C shared library.");
    let destination_path = cmake::build(&interface_directory);
    let lib_path = destination_path.join("lib").join("libsdsl_c.so");
    Ok(lib_path)
}
