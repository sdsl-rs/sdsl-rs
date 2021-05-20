use anyhow::Result;

#[test]
fn test_store_and_from_file() -> Result<()> {
    let tmp_dir = tempdir::TempDir::new("sdsl-rs-tests")?;
    let tmp_directory_path = tmp_dir.path().to_path_buf();
    let path = tmp_directory_path.join("int_vector.bin");

    let mut iv = sdsl::IntVector::<28>::new(5, 12, None)?;
    iv.set(2, 42);
    sdsl::io::store_int_vector_to_file(&iv, &path, true)?;

    let loaded_iv = sdsl::IntVector::<0>::from_file(5, 28, &path)?;
    let result = loaded_iv.get(2);
    let expected = 42;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_store_and_load_from_file() -> Result<()> {
    let tmp_dir = tempdir::TempDir::new("sdsl-rs-tests")?;
    let tmp_directory_path = tmp_dir.path().to_path_buf();
    let path = tmp_directory_path.join("int_vector.bin");

    let mut iv = sdsl::IntVector::<28>::new(5, 12, None)?;
    iv.set(2, 42);
    sdsl::io::store_int_vector_to_file(&iv, &path, true)?;

    let mut loaded_iv = sdsl::IntVector::<0>::new(5, 28, Some(28))?;
    sdsl::io::load_from_file(&mut loaded_iv, &path)?;

    let result = loaded_iv.get(2);
    let expected = 42;
    assert_eq!(result, expected);
    Ok(())
}
