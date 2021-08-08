use anyhow::Result;

#[test]
fn test_get() -> Result<()> {
    let iv = sdsl::int_vector! {1, 1, 7, 1, 1, 2, 3, 4};
    let wt = sdsl::wavelet_trees::WtInt::<sdsl::bit_vectors::BitVector>::from_int_vector(&iv)?;
    let result = wt.get(2);
    let expected = 7;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_rank() -> Result<()> {
    let iv = sdsl::int_vector! {1, 1, 2, 1, 1, 2, 3, 4};
    let wt = sdsl::wavelet_trees::WtInt::<sdsl::bit_vectors::BitVector>::from_int_vector(&iv)?;
    let result = wt.rank(6, 2);
    let expected = 2;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_inverse_select() -> Result<()> {
    let iv = sdsl::int_vector! {1, 1, 2, 1, 1, 2, 3, 4};
    let wt = sdsl::wavelet_trees::WtInt::<sdsl::bit_vectors::BitVector>::from_int_vector(&iv)?;
    let result = wt.inverse_select(3);
    let expected = (1, 2);
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_select() -> Result<()> {
    let iv = sdsl::int_vector! {1, 1, 2, 1, 1, 2, 3, 4};
    let wt = sdsl::wavelet_trees::WtInt::<sdsl::bit_vectors::BitVector>::from_int_vector(&iv)?;
    let result = wt.select(2, 2);
    let expected = 5;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_found_correct_interval_symbols() -> Result<()> {
    let iv = sdsl::int_vector! {4, 3, 3, 2};
    let wt = sdsl::wavelet_trees::WtInt::<sdsl::bit_vectors::BitVector>::from_int_vector(&iv)?;
    let result = wt.interval_symbols(0, 2);
    let result = result.interval_symbols;
    let expected = [3, 4, 0];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_interval_symbols_correct_rank_lower() -> Result<()> {
    let iv = sdsl::int_vector! {1, 1, 2, 1, 1, 2, 3, 4};
    let wt = sdsl::wavelet_trees::WtInt::<sdsl::bit_vectors::BitVector>::from_int_vector(&iv)?;
    let result = wt.interval_symbols(4, 6);
    let result = result.rank_symbols_lower;
    let expected = [3, 1, 0, 0];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_interval_symbols_correct_rank_upper() -> Result<()> {
    let iv = sdsl::int_vector! {1, 1, 2, 1, 1, 2, 3, 4};
    let wt = sdsl::wavelet_trees::WtInt::<sdsl::bit_vectors::BitVector>::from_int_vector(&iv)?;
    let result = wt.interval_symbols(4, 6);
    let result = result.rank_symbols_upper;
    let expected = [4, 2, 0, 0];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_interval_symbols_alphabet_size() -> Result<()> {
    let iv = sdsl::int_vector! {1, 1, 2, 1, 1, 2, 3, 4};
    let wt = sdsl::wavelet_trees::WtInt::<sdsl::bit_vectors::BitVector>::from_int_vector(&iv)?;
    let result = wt.interval_symbols(4, 7);
    let result = result.interval_alphabet_size;
    let expected = 3;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_lex_count_greater() -> Result<()> {
    let iv = sdsl::int_vector! {1, 1, 5, 1, 1, 2, 3, 4};
    let wt = sdsl::wavelet_trees::WtInt::<sdsl::bit_vectors::BitVector>::from_int_vector(&iv)?;
    let result = wt.lex_count(0, 6, 2);
    let result = result.count_greater_symbols;
    let expected = 1;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_lex_count_smaller() -> Result<()> {
    let iv = sdsl::int_vector! {1, 1, 2, 1, 1, 2, 3, 4};
    let wt = sdsl::wavelet_trees::WtInt::<sdsl::bit_vectors::BitVector>::from_int_vector(&iv)?;
    let result = wt.lex_count(0, 6, 2);
    let result = result.count_smaller_symbols;
    let expected = 4;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_range_search_2d_correct_indexes() -> Result<()> {
    let iv = sdsl::int_vector! {1, 1, 2, 1, 1, 2, 3, 4};
    let wt = sdsl::wavelet_trees::WtInt::<sdsl::bit_vectors::BitVector>::from_int_vector(&iv)?;
    let result = wt.range_search_2d(0, 6, 2, 4, true);
    let result = result.point_indexes;
    let expected = [2, 5, 6];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_range_search_2d_correct_values() -> Result<()> {
    let iv = sdsl::int_vector! {1, 1, 2, 1, 1, 2, 3, 4};
    let wt = sdsl::wavelet_trees::WtInt::<sdsl::bit_vectors::BitVector>::from_int_vector(&iv)?;
    let result = wt.range_search_2d(0, 6, 2, 4, true);
    let result = result.point_values;
    let expected = [2, 2, 3];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_alphabet_size() -> Result<()> {
    let iv = sdsl::int_vector! {1, 1, 2, 1, 1, 2, 3, 4};
    let wt = sdsl::wavelet_trees::WtInt::<sdsl::bit_vectors::BitVector>::from_int_vector(&iv)?;
    let result = wt.alphabet_size();
    let expected = 4;
    assert_eq!(result, expected);
    Ok(())
}
