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

    pub fn index(self) -> usize {
        self.0 as usize
    }
}
