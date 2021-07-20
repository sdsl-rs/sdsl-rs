use anyhow::Result;

#[test]
fn test_found_correct_interval_symbols() -> Result<()> {
    #[rustfmt::skip]
    //                                   | 113                |  | 115                      |
    let bv = sdsl::bit_vector! {1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0};
    let wt = sdsl::wavelet_trees::WtHuff::<sdsl::bit_vectors::BitVector>::from_bit_vector(&bv)?;
    let result = wt.interval_symbols(0, 2);
    let result = result.interval_symbols;
    let expected = [113, 115];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_interval_symbols_correct_rank_lower() -> Result<()> {
    #[rustfmt::skip]
    //                                   | 113                |  | 113                |  | 115                |
    let bv = sdsl::bit_vector! {1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0};
    let wt = sdsl::wavelet_trees::WtHuff::<sdsl::bit_vectors::BitVector>::from_bit_vector(&bv)?;
    let result = wt.interval_symbols(1, 2);
    let result = result.rank_symbols_lower;
    let expected = [1, 0];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_interval_symbols_correct_rank_upper() -> Result<()> {
    #[rustfmt::skip]
    //                                   | 113                |  | 113                |  | 115                |  | 115                |
    let bv = sdsl::bit_vector! {1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0};
    let wt = sdsl::wavelet_trees::WtHuff::<sdsl::bit_vectors::BitVector>::from_bit_vector(&bv)?;
    let result = wt.interval_symbols(0, 3);
    let result = result.rank_symbols_upper;
    let expected = [2, 1];
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_interval_symbols_alphabet_size() -> Result<()> {
    #[rustfmt::skip]
    //                                   | 113                |  | 113                |  | 115                |
    let bv = sdsl::bit_vector! {1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0};
    let wt = sdsl::wavelet_trees::WtHuff::<sdsl::bit_vectors::BitVector>::from_bit_vector(&bv)?;
    let result = wt.interval_symbols(0, 1);
    let result = result.interval_alphabet_size;
    let expected = 1;
    assert_eq!(result, expected);
    Ok(())
}
