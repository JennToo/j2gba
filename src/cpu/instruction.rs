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
}
