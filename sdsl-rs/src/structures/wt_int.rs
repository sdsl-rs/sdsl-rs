use crate::structures;

pub struct WtInt<
    BitVector = structures::bit_vector::BitVector,
    Rank = structures::bit_vector::Rank,
    Select = structures::bit_vector::Select,
    SelectZero = structures::bit_vector::SelectZero,
> {
    _bit_vector: BitVector,
    _rank: Rank,
    _select: Select,
    _select_zero: SelectZero,
}

impl<BitVector, Rank, Select, SelectZero> WtInt<BitVector, Rank, Select, SelectZero> {
    pub fn new(bit_vector: BitVector, rank: Rank, select: Select, select_zero: SelectZero) -> Self {
        Self {
            _bit_vector: bit_vector,
            _rank: rank,
            _select: select,
            _select_zero: select_zero,
        }
    }
}
