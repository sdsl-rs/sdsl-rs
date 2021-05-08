pub fn foo() {
    let bv = sdsl::bit_vector::BitVector::new(1);
    let rank = sdsl::bit_vector::Rank {};
    let select = sdsl::bit_vector::Select {};
    let select_zero = sdsl::bit_vector::SelectZero {};
    let _wt =
        sdsl::wt_int::WtInt::<sdsl::bit_vector::BitVector>::new(bv, rank, select, select_zero);
    println!(" ");
}
