use super::Register;
use crate::util::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ConditionCode {
    Equal,
    NotEqual,
    CarrySet,
    CarryClear,
    Minus,
    Plus,
    OverflowSet,
    OverflowClear,
    UnsignedHigher,
    UnsignedLowerOrSame,
    SignedGreaterThanOrEqual,
    SignedLessThan,
    SignedGreaterThan,
    SignedLessThanOrEqual,
    Always,
    Never,
}

impl ConditionCode {
    pub fn from_bits(bits: u8) -> ConditionCode {
        match bits {
            0b0000 => ConditionCode::Equal,
            0b0001 => ConditionCode::NotEqual,
            0b0010 => ConditionCode::CarrySet,
            0b0011 => ConditionCode::CarryClear,
            0b0100 => ConditionCode::Minus,
            0b0101 => ConditionCode::Plus,
            0b0110 => ConditionCode::OverflowSet,
            0b0111 => ConditionCode::OverflowClear,
            0b1000 => ConditionCode::UnsignedHigher,
            0b1001 => ConditionCode::UnsignedLowerOrSame,
            0b1010 => ConditionCode::SignedGreaterThanOrEqual,
            0b1011 => ConditionCode::SignedLessThan,
            0b1100 => ConditionCode::SignedGreaterThan,
            0b1101 => ConditionCode::SignedLessThanOrEqual,
            0b1110 => ConditionCode::Always,
            0b1111 => ConditionCode::Never,
            _ => panic!("Only the least significant 4 bits may be set"),
        }
    }
}

impl std::fmt::Display for ConditionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ConditionCode::Equal => write!(f, "eq"),
            ConditionCode::NotEqual => write!(f, "ne"),
            ConditionCode::CarrySet => write!(f, "cs"),
            ConditionCode::CarryClear => write!(f, "cc"),
            ConditionCode::Minus => write!(f, "mi"),
            ConditionCode::Plus => write!(f, "pl"),
            ConditionCode::OverflowSet => write!(f, "vs"),
            ConditionCode::OverflowClear => write!(f, "vc"),
            ConditionCode::UnsignedHigher => write!(f, "hi"),
            ConditionCode::UnsignedLowerOrSame => write!(f, "ls"),
            ConditionCode::SignedGreaterThanOrEqual => write!(f, "ge"),
            ConditionCode::SignedLessThan => write!(f, "lt"),
            ConditionCode::SignedGreaterThan => write!(f, "gt"),
            ConditionCode::SignedLessThanOrEqual => write!(f, "le"),
            ConditionCode::Always => write!(f, "al"),
            ConditionCode::Never => write!(f, "nv"),
        }
    }
}

// TODO: Refactor into a better structure
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ShifterOperand {
    Immediate { value: u8, rotate: u8 },
    LogicalLeftShiftImmediate { source: Register, shifter: u8 },
    LogicalLeftShiftRegister { source: Register, shifter: Register },
    LogicalRightShiftImmediate { source: Register, shifter: u8 },
    LogicalRightShiftRegister { source: Register, shifter: Register },
    ArithmeticRightShiftImmediate { source: Register, shifter: u8 },
    ArithmeticRightShiftRegister { source: Register, shifter: Register },
    RotateRightImmediate { source: Register, rotate: u8 },
    RotateRightRegister { source: Register, rotate: Register },
    RotateRightExtended { source: Register },
}

impl ShifterOperand {
    pub fn from_immediate(bits: u16) -> Self {
        ShifterOperand::Immediate {
            value: bits.get_bits(Offset(0), Length(8)) as u8,
            rotate: bits.get_bits(Offset(8), Length(4)) as u8,
        }
    }

    pub fn from_register_operand(bits: u16) -> Self {
        let discriminant = bits.get_bits(Offset(4), Length(3));

        let source = bits.get_register(Offset(0));
        match discriminant {
            0b000 => ShifterOperand::LogicalLeftShiftImmediate {
                source,
                shifter: bits.get_bits(Offset(7), Length(4)) as u8,
            },
            0b001 => ShifterOperand::LogicalLeftShiftRegister {
                source,
                shifter: bits.get_register(Offset(8)),
            },
            0b010 => ShifterOperand::LogicalRightShiftImmediate {
                source,
                shifter: bits.get_bits(Offset(7), Length(4)) as u8,
            },
            0b011 => ShifterOperand::LogicalRightShiftRegister {
                source,
                shifter: bits.get_register(Offset(8)),
            },
            0b100 => ShifterOperand::ArithmeticRightShiftImmediate {
                source,
                shifter: bits.get_bits(Offset(7), Length(4)) as u8,
            },
            0b101 => ShifterOperand::ArithmeticRightShiftRegister {
                source,
                shifter: bits.get_register(Offset(8)),
            },
            0b110 => {
                let rotate = bits.get_bits(Offset(7), Length(4)) as u8;
                if rotate == 0 {
                    ShifterOperand::RotateRightExtended { source }
                } else {
                    ShifterOperand::RotateRightImmediate { source, rotate }
                }
            }
            0b111 => ShifterOperand::RotateRightRegister {
                source,
                rotate: bits.get_register(Offset(8)),
            },
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_condition_codes() {
        let expected = [
            ConditionCode::Equal,
            ConditionCode::NotEqual,
            ConditionCode::CarrySet,
            ConditionCode::CarryClear,
            ConditionCode::Minus,
            ConditionCode::Plus,
            ConditionCode::OverflowSet,
            ConditionCode::OverflowClear,
            ConditionCode::UnsignedHigher,
            ConditionCode::UnsignedLowerOrSame,
            ConditionCode::SignedGreaterThanOrEqual,
            ConditionCode::SignedLessThan,
            ConditionCode::SignedGreaterThan,
            ConditionCode::SignedLessThanOrEqual,
            ConditionCode::Always,
            ConditionCode::Never,
        ];

        let decoded: Vec<_> = (0..16).map(ConditionCode::from_bits).collect();
        assert_eq!(decoded.as_slice(), expected);

        let stringified: Vec<_> = expected.iter().map(|x| format!("{}", x)).collect();
        assert_eq!(
            stringified.as_slice(),
            &[
                "eq", "ne", "cs", "cc", "mi", "pl", "vs", "vc", "hi", "ls", "ge", "lt", "gt", "le",
                "al", "nv"
            ]
        );
    }

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
            0b00000_110_0101,
        ];
        let decoded_shifters: Vec<_> = raw_shifters
            .iter()
            .map(|x| ShifterOperand::from_register_operand(*x))
            .collect();
        assert_eq!(
            decoded_shifters.as_slice(),
            [
                ShifterOperand::LogicalLeftShiftImmediate {
                    source: Register(5),
                    shifter: 9
                },
                ShifterOperand::LogicalLeftShiftRegister {
                    source: Register(5),
                    shifter: Register(11)
                },
                ShifterOperand::LogicalRightShiftImmediate {
                    source: Register(5),
                    shifter: 9
                },
                ShifterOperand::LogicalRightShiftRegister {
                    source: Register(5),
                    shifter: Register(11)
                },
                ShifterOperand::ArithmeticRightShiftImmediate {
                    source: Register(5),
                    shifter: 9
                },
                ShifterOperand::ArithmeticRightShiftRegister {
                    source: Register(5),
                    shifter: Register(11)
                },
                ShifterOperand::RotateRightImmediate {
                    source: Register(5),
                    rotate: 9
                },
                ShifterOperand::RotateRightRegister {
                    source: Register(5),
                    rotate: Register(11)
                },
                ShifterOperand::RotateRightExtended {
                    source: Register(5),
                }
            ]
        );
    }
}
