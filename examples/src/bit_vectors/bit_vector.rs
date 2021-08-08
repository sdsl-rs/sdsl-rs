use anyhow::Result;

#[test]
fn test_cloned_vector_does_not_share_elements() -> Result<()> {
    let bv = sdsl::bit_vectors::BitVector::new(5, 1)?;
    let mut bv_clone = bv.clone();
    bv_clone.set(1, 0);

    let result = (bv.get(1), bv_clone.get(1));
    let expected = (1, 0);
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_len_gives_number_of_elements() -> Result<()> {
    let bv = sdsl::bit_vectors::BitVector::new(5, 1)?;
    let result = bv.len();
    let expected = 5;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_correct_len_after_resize() -> Result<()> {
    let mut bv = sdsl::bit_vectors::BitVector::new(5, 1)?;
    bv.resize(6);

    let result = bv.len();
    let expected = 6;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_resize_truncates_vector() -> Result<()> {
    let mut bv = sdsl::bit_vector! {1, 0, 1};
    bv.resize(2);

    let result: Vec<_> = bv.iter().collect();
    let expected = vec![1, 0];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_capacity_sufficient_multiple_of_64() -> Result<()> {
    let bv = sdsl::bit_vectors::BitVector::new(65, 1)?;
    let result = bv.capacity();
    let expected = 128;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_literal_macro() -> Result<()> {
    let bv = sdsl::bit_vector! {1, 0, 1};

    let result = (bv.get(0), bv.get(1), bv.get(2));
    let expected = (1, 0, 1);
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_bit_size() -> Result<()> {
    let bv = sdsl::bit_vector! {1, 1, 0, 1};

    let result = bv.bit_size();
    let expected = 4;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_get_int() -> Result<()> {
    let bv = sdsl::bit_vector! {1, 1, 0, 0, 1};

    let result = bv.get_int(0, 5);
    let expected = 19;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_set_int() -> Result<()> {
    let mut bv = sdsl::bit_vector! {0, 0, 0, 0, 0};
    bv.set_int(0, 19, 5);

    let result = bv.clone();
    let expected = sdsl::bit_vector! {1, 1, 0, 0, 1};
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_iter() -> Result<()> {
    let bv = sdsl::bit_vector! {1, 1, 0, 1};

    let result = bv.iter().collect::<Vec<_>>();
    let expected = vec![1, 1, 0, 1];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_into_iter() -> Result<()> {
    let bv = sdsl::bit_vector! {1, 1, 0, 1};

    let result = bv.into_iter().collect::<Vec<_>>();
    let expected = vec![1, 1, 0, 1];
    assert_eq!(result, expected);
    Ok(())
}
