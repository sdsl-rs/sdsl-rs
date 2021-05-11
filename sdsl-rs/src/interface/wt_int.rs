use crate::interface;

pub struct WtInt<
    BitVector = interface::bit_vector::BitVector,
    Rank = interface::bit_vector::Rank,
    Select = interface::bit_vector::Select,
    SelectZero = interface::bit_vector::SelectZero,
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
