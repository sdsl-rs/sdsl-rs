use anyhow::{format_err, Result};
use std::io::Write;

pub mod interface;

pub fn setup(out_directory: &std::path::PathBuf) -> Result<std::path::PathBuf> {
    let template_directory = out_directory.join("sdsl-c-template");
    if template_directory.exists() {
        log::debug!("sdsl-c template setup complete.");
        return Ok(template_directory);
    }
    log::debug!("Setting up sdsl-c template.");

    let archive_path = out_directory.join("sdsl-c-template.zip");
    let bytes = include_bytes!("sdsl-c-template.zip");
    {
        let mut file = std::fs::File::create(&archive_path)?;
        file.write_all(bytes)?;
        file.sync_all()?;
    }

    let unpack_directory = out_directory.join("sdsl-c-template-tmp");
    extract_zip(&archive_path, &unpack_directory)?;
    std::fs::remove_file(&archive_path)?;

    let entry = std::fs::read_dir(&unpack_directory)?
        .next()
        .ok_or(format_err!("Unpacked sdsl-c-template is empty."))?;
    let entry = entry?;
    let top_level_directory = entry.path();

    std::fs::rename(&top_level_directory, &template_directory)?;
    std::fs::remove_dir_all(&unpack_directory)?;

    Ok(template_directory)
}

fn extract_zip(
    archive_path: &std::path::PathBuf,
    destination_directory: &std::path::PathBuf,
) -> Result<std::path::PathBuf> {
    let file = std::fs::File::open(&archive_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let extracted_directory = destination_directory.join(
        archive
            .by_index(0)?
            .enclosed_name()
            .ok_or(format_err!(
                "Archive is unexpectedly empty: {}",
                archive_path.display()
            ))?
            .to_path_buf(),
    );

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let output_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let output_path = destination_directory.join(output_path);

        if (&*file.name()).ends_with('/') {
            std::fs::create_dir_all(&output_path)?;
        } else {
            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(&parent)?;
                }
            }
            let mut output_file = std::fs::File::create(&output_path)?;
            std::io::copy(&mut file, &mut output_file)?;
        }
    }
    Ok(extracted_directory)
}
