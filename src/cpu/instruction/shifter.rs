use crate::cpu::instruction::{DecodeError, Result};
use crate::cpu::Register;
use crate::util::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ShifterOperand {
    Immediate {
        value: u8,
        rotate: u8,
    },
    Shift {
        operation: ShiftOperation,
        source: Register,
        operand: ShiftOperand,
    },
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ShiftOperation {
    LogicalLeft,
    LogicalRight,
    ArithmeticRight,
    RotateRight,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ShiftOperand {
    Register(Register),
    Value(u8),
}

impl ShifterOperand {
    pub fn from_immediate(bits: u16) -> Self {
        ShifterOperand::Immediate {
            value: bits.get_bits(Offset(0), Length(8)) as u8,
            rotate: bits.get_bits(Offset(8), Length(4)) as u8,
        }
    }

    pub fn from_register_operand(bits: u16) -> Result<Self> {
        let operand = if bits.is_flag_set(Offset(4)) {
            if bits.is_flag_set(Offset(7)) {
                return Err(DecodeError::InvalidShifterOperand(bits));
            }
            ShiftOperand::Register(bits.get_register(Offset(8)))
        } else {
            ShiftOperand::Value(bits.get_bits(Offset(7), Length(4)) as u8)
        };

        let operation = match bits.get_bits(Offset(5), Length(2)) {
            0b00 => ShiftOperation::LogicalLeft,
            0b01 => ShiftOperation::LogicalRight,
            0b10 => ShiftOperation::ArithmeticRight,
            0b11 => ShiftOperation::RotateRight,
            _ => unreachable!(),
        };

        let source = bits.get_register(Offset(0));

        Ok(ShifterOperand::Shift {
            operation,
            operand,
            source,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shifter_operand() {
        let result = ShifterOperand::from_immediate(0b11111010_11001100);
        assert_eq!(
            result,
            ShifterOperand::Immediate {
                value: 0b11001100,
                rotate: 0b1010
            }
        );

        let raw_shifters = [
            0b11001_000_0101,
            0b10110_001_0101,
            0b11001_010_0101,
            0b10110_011_0101,
            0b11001_100_0101,
            0b10110_101_0101,
            0b11001_110_0101,
            0b10110_111_0101,
            0b10111_001_0101,
        ];
        let decoded_shifters: Vec<_> = raw_shifters
            .iter()
            .map(|x| ShifterOperand::from_register_operand(*x))
            .collect();
        assert_eq!(
            decoded_shifters.as_slice(),
            [
                Ok(ShifterOperand::Shift {
                    source: Register(5),
                    operation: ShiftOperation::LogicalLeft,
                    operand: ShiftOperand::Value(9)
                }),
                Ok(ShifterOperand::Shift {
                    source: Register(5),
                    operation: ShiftOperation::LogicalLeft,
                    operand: ShiftOperand::Register(Register(11))
                }),
                Ok(ShifterOperand::Shift {
                    source: Register(5),
                    operation: ShiftOperation::LogicalRight,
                    operand: ShiftOperand::Value(9)
                }),
                Ok(ShifterOperand::Shift {
                    source: Register(5),
                    operation: ShiftOperation::LogicalRight,
                    operand: ShiftOperand::Register(Register(11))
                }),
                Ok(ShifterOperand::Shift {
                    source: Register(5),
                    operation: ShiftOperation::ArithmeticRight,
                    operand: ShiftOperand::Value(9)
                }),
                Ok(ShifterOperand::Shift {
                    source: Register(5),
                    operation: ShiftOperation::ArithmeticRight,
                    operand: ShiftOperand::Register(Register(11))
                }),
                Ok(ShifterOperand::Shift {
                    source: Register(5),
                    operation: ShiftOperation::RotateRight,
                    operand: ShiftOperand::Value(9)
                }),
                Ok(ShifterOperand::Shift {
                    source: Register(5),
                    operation: ShiftOperation::RotateRight,
                    operand: ShiftOperand::Register(Register(11))
                }),
                Err(DecodeError::InvalidShifterOperand(0b10111_001_0101)),
            ]
        );
    }
}
