mod instruction;

const REGISTER_COUNT: usize = 16;
const REGISTER_MASK: u32 = (2 << REGISTER_COUNT) - 1;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Register(u8);

impl Register {
    pub fn from_index(index: usize) -> Self {
        assert!(index < REGISTER_COUNT);
        Register(index as u8)
    }
    pub fn from_bit_offset(data: u32, bit_offset: u32) -> Self {
        Register(((data >> bit_offset) & REGISTER_MASK) as u8)
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_register() {
        assert_eq!(Register::from_index(12).index(), 12);

        assert_eq!(
            Register::from_bit_offset(0b1111_0000, 4),
            Register::from_index(15)
        );
    }
}
