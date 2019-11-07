use super::ShifterOperand;
use crate::cpu::instruction::Result;
use crate::cpu::Register;
use crate::util::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DataOpcode {
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

impl DataOpcode {
    pub fn from_bits(bits: u8) -> DataOpcode {
        match bits {
            0b0000 => DataOpcode::And,
            0b0001 => DataOpcode::ExclusiveOr,
            0b0010 => DataOpcode::Subtract,
            0b0011 => DataOpcode::ReverseSubtract,
            0b0100 => DataOpcode::Add,
            0b0101 => DataOpcode::AddWithCarry,
            0b0110 => DataOpcode::SubtractWithCarry,
            0b0111 => DataOpcode::ReverseSubtractWithCarry,
            0b1000 => DataOpcode::Test,
            0b1001 => DataOpcode::TestEquivalence,
            0b1010 => DataOpcode::Compare,
            0b1011 => DataOpcode::CompareNegated,
            0b1100 => DataOpcode::Or,
            0b1101 => DataOpcode::Move,
            0b1110 => DataOpcode::BitClear,
            0b1111 => DataOpcode::MoveNot,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for DataOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DataOpcode::And => write!(f, "and"),
            DataOpcode::ExclusiveOr => write!(f, "eor"),
            DataOpcode::Subtract => write!(f, "sub"),
            DataOpcode::ReverseSubtract => write!(f, "rsb"),
            DataOpcode::Add => write!(f, "add"),
            DataOpcode::AddWithCarry => write!(f, "adc"),
            DataOpcode::SubtractWithCarry => write!(f, "sbc"),
            DataOpcode::ReverseSubtractWithCarry => write!(f, "rsc"),
            DataOpcode::Test => write!(f, "tst"),
            DataOpcode::TestEquivalence => write!(f, "teq"),
            DataOpcode::Compare => write!(f, "cmp"),
            DataOpcode::CompareNegated => write!(f, "cmn"),
            DataOpcode::Or => write!(f, "orr"),
            DataOpcode::Move => write!(f, "mov"),
            DataOpcode::BitClear => write!(f, "bic"),
            DataOpcode::MoveNot => write!(f, "mvn"),
        }
    }
}

#[test]
fn test_decode() {
    let expected = [
        DataOpcode::And,
        DataOpcode::ExclusiveOr,
        DataOpcode::Subtract,
        DataOpcode::ReverseSubtract,
        DataOpcode::Add,
        DataOpcode::AddWithCarry,
        DataOpcode::SubtractWithCarry,
        DataOpcode::ReverseSubtractWithCarry,
        DataOpcode::Test,
        DataOpcode::TestEquivalence,
        DataOpcode::Compare,
        DataOpcode::CompareNegated,
        DataOpcode::Or,
        DataOpcode::Move,
        DataOpcode::BitClear,
        DataOpcode::MoveNot,
    ];

    let decoded: Vec<_> = (0..16).map(DataOpcode::from_bits).collect();

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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct DataInstruction {
    operand1: Register,
    operand2: ShifterOperand,
    destination: Register,
    opcode: DataOpcode,
    set_condition_codes: bool,
}

impl DataInstruction {
    pub fn from_bits(bits: u32) -> Result<Self> {
        let is_immediate = bits.is_flag_set(Offset(25));
        let set_condition_codes = bits.is_flag_set(Offset(20));
        let destination = bits.get_register(Offset(12));
        let operand1 = bits.get_register(Offset(16));
        let opcode = DataOpcode::from_bits(bits.get_bits(Offset(21), Length(4)) as u8);
        let operand2 = if is_immediate {
            ShifterOperand::from_immediate(bits as u16)
        } else {
            ShifterOperand::from_register_operand(bits as u16)?
        };

        Ok(DataInstruction {
            operand1,
            operand2,
            destination,
            opcode,
            set_condition_codes,
        })
    }
}

#[test]
fn test_data_instruction_immediate() {
    let decoded = DataInstruction::from_bits(0b1_0101_1_1001_1101_101011101011).unwrap();
    assert_eq!(
        decoded,
        DataInstruction {
            set_condition_codes: true,
            operand1: Register::from_index(0b1001),
            operand2: ShifterOperand::from_immediate(0b101011101011),
            destination: Register::from_index(0b1101),
            opcode: DataOpcode::AddWithCarry,
        }
    );
}

#[test]
fn test_data_instruction_shifter() {
    let decoded = DataInstruction::from_bits(0b0_0101_1_1001_1101_101011101011).unwrap();
    assert_eq!(
        decoded,
        DataInstruction {
            set_condition_codes: true,
            operand1: Register::from_index(0b1001),
            operand2: ShifterOperand::from_register_operand(0b101011101011).unwrap(),
            destination: Register::from_index(0b1101),
            opcode: DataOpcode::AddWithCarry,
        }
    );
}
