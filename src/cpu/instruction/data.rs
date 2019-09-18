use crate::cpu::instruction::Result;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DataOperation {
    And,
    ExclusiveOr,
    Subtract,
    ReverseSubtract,
    Add,
    AddWithCarry,
    SubtractWithCarry,
    ReverseSubtractWithCarry,
    Test,
    TestEquivalence,
    Compare,
    CompareNegated,
    Or,
    Move,
    BitClear,
    MoveNot,
}

impl DataOperation {
    pub fn from_bits(bits: u32) -> Result<DataOperation> {
        unimplemented!()
    }
}
