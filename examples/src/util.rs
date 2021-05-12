use anyhow::Result;

#[test]
fn test_set_to_value() -> Result<()> {
    let mut iv = sdsl::IntVector::<28>::new(5, 12, None)?;
    sdsl::util::set_to_value(&mut iv, 42);
    let result = iv.get(2);
    let expect = 42;
    assert_eq!(result, expect);
    Ok(())
}

#[test]
fn test_set_to_id() -> Result<()> {
    let mut iv = sdsl::IntVector::<28>::new(5, 12, None)?;
    sdsl::util::set_to_id(&mut iv);
    let result = iv.get(2);
    let expect = 2;
    assert_eq!(result, expect);
    Ok(())
}

#[test]
fn test_set_random_bits() -> Result<()> {
    let mut iv = sdsl::IntVector::<28>::new(5, 12, None)?;
    sdsl::util::set_random_bits(&mut iv);
    // TODO: Freeze seed in test.
    Ok(())
}

#[test]
fn test_modulus() -> Result<()> {
    let mut iv = sdsl::IntVector::<28>::new(5, 12, None)?;
    sdsl::util::modulus(&mut iv, 10);
    let result = iv.get(3);
    let expected = 2;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_bit_compress() -> Result<()> {
    let mut iv = sdsl::IntVector::<0>::new(5, 12, Some(64))?;
    sdsl::util::bit_compress(&mut iv);
    let result = iv.width();
    let expected = 4;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_expand_width() -> Result<()> {
    let mut iv = sdsl::IntVector::<0>::new(5, 12, Some(20))?;
    sdsl::util::expand_width(&mut iv, 21);
    let result = iv.width();
    let expected = 21;
    assert_eq!(result, expected);
    Ok(())
}
