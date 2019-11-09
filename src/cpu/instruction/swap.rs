use crate::cpu::Register;
use crate::util::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SwapSize {
    Byte,
    Word,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Swap {
    source: Register,
    destination: Register,
    base: Register,
    size: SwapSize,
}

impl Swap {
    fn from_bits(bits: u32) -> Swap {
        Swap {
            source: bits.get_register(Offset(0)),
            destination: bits.get_register(Offset(12)),
            base: bits.get_register(Offset(16)),
            size: if bits.is_flag_set(Offset(22)) {
                SwapSize::Byte
            } else {
                SwapSize::Word
            },
        }
    }
}

#[test]
fn decode() {
    let decoded = Swap::from_bits(0b1_00_1001_1100_00001001_1011);

    assert_eq!(
        decoded,
        Swap {
            source: Register::from_index(0b1011),
            destination: Register::from_index(0b1100),
            base: Register::from_index(0b1001),
            size: SwapSize::Byte,
        }
    );
}
