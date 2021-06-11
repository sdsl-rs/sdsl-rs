use anyhow::Result;

#[test]
fn test_rank_p0() -> Result<()> {
    let bv = sdsl::bit_vector! {0, 1, 0, 1, 0, 0, 0};
    let rs = sdsl::rank_supports::RankSupportV::<sdsl::bit_patterns::P0>::new(&bv)?;

    let result = rs.rank(5);
    let expected = 3;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_rank_p1() -> Result<()> {
    let bv = sdsl::bit_vector! {1, 1, 0, 1, 0, 0, 0};
    let rs = sdsl::rank_supports::RankSupportV::<sdsl::bit_patterns::P1>::new(&bv)?;

    let result = rs.rank(5);
    let expected = 3;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_rank_p10() -> Result<()> {
    let bv = sdsl::bit_vector! {0, 1, 0, 1, 0, 0, 0};
    let rs = sdsl::rank_supports::RankSupportV::<sdsl::bit_patterns::P10>::new(&bv)?;

    let result = rs.rank(4);
    let expected = 1;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_rank_p01() -> Result<()> {
    let bv = sdsl::bit_vector! {0, 1, 0, 1, 0, 0, 0};
    let rs = sdsl::rank_supports::RankSupportV::<sdsl::bit_patterns::P01>::new(&bv)?;

    let result = rs.rank(4);
    let expected = 2;
    assert_eq!(result, expected);
    Ok(())
}
