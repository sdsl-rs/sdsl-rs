use anyhow::Result;

#[cfg(test)]
mod int_vector;

pub fn foo() -> Result<()> {
    // hi        n

    let iv = sdsl::int_vector::IntVector::<0>::new(5, 42, Some(64))?;
    println!("get: {}", iv.get(2));
    println!("is_empty: {}", iv.is_empty());
    println!("len: {}", iv.len());

    // let bv = sdsl::bit_vector::BitVector::new(1)?;
    // let rank = sdsl::bit_vector::Rank {};
    // let select = sdsl::bit_vector::Select {};
    // let select_zero = sdsl::bit_vector::SelectZero {};
    // let _wt =
    //     sdsl::wt_int::WtInt::<sdsl::bit_vector::BitVector>::new(bv, rank, select, select_zero);
    // println!(" ");
    Ok(())
}
