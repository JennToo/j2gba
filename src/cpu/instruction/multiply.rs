use crate::cpu::Register;
use crate::util::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Multiply {
    long: bool,
    signed: bool,
    accumulate: bool,
    set_condition_codes: bool,
    operand1: Register,
    operand2: Register,
    operand3: Register,
    destination: Register,
}

impl Multiply {
    pub fn from_bits(bits: u32) -> Multiply {
        unimplemented!();
    }
}

#[test]
fn test_decode_short()
{
    let decoded = Multiply::from_bits(0b0_0_1_1_0011_1010_1001_1001_1100);
    assert_eq!(decoded, Multiply {
        long: false,
        signed: false,
        accumulate: true,
        set_condition_codes: true,

        operand1: Register::from_index(0b1010),
        operand2: Register::from_index(0b1001),
        operand3: Register::from_index(0b1100),
        destination: Register::from_index(0b0011),
    });
}
