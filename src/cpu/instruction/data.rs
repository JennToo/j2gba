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
    pub fn from_bits(bits: u8) -> DataOperation {
        match bits {
            0b0000 => DataOperation::And,
            0b0001 => DataOperation::ExclusiveOr,
            0b0010 => DataOperation::Subtract,
            0b0011 => DataOperation::ReverseSubtract,
            0b0100 => DataOperation::Add,
            0b0101 => DataOperation::AddWithCarry,
            0b0110 => DataOperation::SubtractWithCarry,
            0b0111 => DataOperation::ReverseSubtractWithCarry,
            0b1000 => DataOperation::Test,
            0b1001 => DataOperation::TestEquivalence,
            0b1010 => DataOperation::Compare,
            0b1011 => DataOperation::CompareNegated,
            0b1100 => DataOperation::Or,
            0b1101 => DataOperation::Move,
            0b1110 => DataOperation::BitClear,
            0b1111 => DataOperation::MoveNot,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for DataOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DataOperation::And => write!(f, "and"),
            DataOperation::ExclusiveOr => write!(f, "eor"),
            DataOperation::Subtract => write!(f, "sub"),
            DataOperation::ReverseSubtract => write!(f, "rsb"),
            DataOperation::Add => write!(f, "add"),
            DataOperation::AddWithCarry => write!(f, "adc"),
            DataOperation::SubtractWithCarry => write!(f, "sbc"),
            DataOperation::ReverseSubtractWithCarry => write!(f, "rsc"),
            DataOperation::Test => write!(f, "tst"),
            DataOperation::TestEquivalence => write!(f, "teq"),
            DataOperation::Compare => write!(f, "cmp"),
            DataOperation::CompareNegated => write!(f, "cmn"),
            DataOperation::Or => write!(f, "orr"),
            DataOperation::Move => write!(f, "mov"),
            DataOperation::BitClear => write!(f, "bic"),
            DataOperation::MoveNot => write!(f, "mvn"),
        }
    }
}

#[test]
fn test_decode() {
    let expected = [
        DataOperation::And,
        DataOperation::ExclusiveOr,
        DataOperation::Subtract,
        DataOperation::ReverseSubtract,
        DataOperation::Add,
        DataOperation::AddWithCarry,
        DataOperation::SubtractWithCarry,
        DataOperation::ReverseSubtractWithCarry,
        DataOperation::Test,
        DataOperation::TestEquivalence,
        DataOperation::Compare,
        DataOperation::CompareNegated,
        DataOperation::Or,
        DataOperation::Move,
        DataOperation::BitClear,
        DataOperation::MoveNot,
    ];

    let decoded: Vec<_> = (0..16).map(DataOperation::from_bits).collect();

    for (e, d) in expected.iter().zip(decoded.iter()) {
        assert_eq!(e, d);
    }
    let stringified: Vec<_> = expected.iter().map(|x| format!("{}", x)).collect();
    assert_eq!(
        stringified.as_slice(),
        &[
            "and", "eor", "sub", "rsb", "add", "adc", "sbc", "rsc", "tst", "teq", "cmp", "cmn",
            "orr", "mov", "bic", "mvn"
        ]
    );
}
