use anyhow::Result;

#[test]
fn test_select_p0() -> Result<()> {
    let bv = sdsl::bit_vector! {0, 1, 0, 1, 0, 0, 0};
    let ss = sdsl::SelectSupportMcl::<sdsl::bit_patterns::P0>::new(&bv)?;

    let result = ss.select(3);
    let expected = 4;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_select_p1() -> Result<()> {
    let bv = sdsl::bit_vector! {1, 1, 0, 1, 0, 1, 0};
    let ss = sdsl::SelectSupportMcl::<sdsl::bit_patterns::P1>::new(&bv)?;

    let result = ss.select(4);
    let expected = 5;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_select_p10() -> Result<()> {
    let bv = sdsl::bit_vector! {0, 1, 0, 0, 1, 0, 0, 0};
    let ss = sdsl::SelectSupportMcl::<sdsl::bit_patterns::P10>::new(&bv)?;

    let result = ss.select(2);
    let expected = 5;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_select_p01() -> Result<()> {
    let bv = sdsl::bit_vector! {0, 1, 0, 0, 1, 0, 0, 0};
    let ss = sdsl::SelectSupportMcl::<sdsl::bit_patterns::P01>::new(&bv)?;

    let result = ss.select(2);
    let expected = 4;
    assert_eq!(result, expected);
    Ok(())
}
