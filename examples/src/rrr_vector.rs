use anyhow::Result;

#[test]
fn test_from_file() -> Result<()> {
    let tmp_dir = tempdir::TempDir::new("sdsl-rs-tests")?;
    let tmp_directory_path = tmp_dir.path().to_path_buf();
    let path = tmp_directory_path.join("rrr_vector.bin");

    let bv = sdsl::bit_vector! {1, 1, 0, 1};
    let rv = sdsl::RrrVector::<sdsl::IntVector<0>, 10, 2>::new(&bv)?;
    sdsl::io::store_to_file(&rv, &path)?;

    let rv_loaded = sdsl::RrrVector::<sdsl::IntVector<0>, 10, 2>::from_file(&path)?;

    let result: Vec<_> = rv_loaded.iter_bv().collect();
    let expected = vec![1, 1, 0, 1];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_get_int() -> Result<()> {
    let bv = sdsl::bit_vector! {1, 1, 0, 1};
    let rv = sdsl::RrrVector::<sdsl::IntVector<0>, 10, 2>::new(&bv)?;

    let result = rv.get_int(1, 3);
    let expected = 5;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_get_bv_element() -> Result<()> {
    let bv = sdsl::bit_vector! {1, 1, 0, 1};
    let rv = sdsl::RrrVector::<sdsl::IntVector<0>, 10, 2>::new(&bv)?;

    let result = rv.get_bv_element(2);
    let expected = 0;
    assert_eq!(result, expected);
    Ok(())
}
