use anyhow::Result;

#[test]
fn test_() -> Result<()> {
    //  b
    let bv = sdsl::bit_vector! {1, 1, 0, 1};
    let rv = sdsl::RrrVector::<sdsl::IntVector<0>, 10, 2>::new(&bv)?;
    Ok(())
}
