use anyhow::Result;

#[test]
fn test_() -> Result<()> {
    let bv = sdsl::bit_vector! {1, 0, 0, 0, 1, 1, 1};
    let wt = sdsl::wavelet_trees::WtHuff::<sdsl::bit_vectors::BitVector>::from_bit_vector(&bv)?;
    Ok(())
}
