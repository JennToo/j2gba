use crate::cpu::Register;

pub trait GetBits: Sized {
    fn get_bits(self: Self, offset: Offset<Self>, length: Length<Self>) -> Self;
    fn get_register(self: Self, offset: Offset<Self>) -> Register;
    fn is_flag_set(self: Self, offset: Offset<Self>) -> bool;
}

pub struct Offset<T>(pub T);
pub struct Length<T>(pub T);

// I can't figure out how to have a generic impl of this
macro_rules! impl_get_bits {
    ($kind:ty) => {
        impl GetBits for $kind {
            fn get_bits(self: Self, offset: Offset<Self>, length: Length<Self>) -> Self {
                let mask = (1 << length.0) - 1;
                (self >> offset.0) & mask
            }

            fn get_register(self: Self, offset: Offset<Self>) -> Register {
                Register::from_index(self.get_bits(offset, Length(4)) as usize)
            }

            fn is_flag_set(self: Self, offset: Offset<Self>) -> bool {
                self.get_bits(offset, Length(1)) == 1
            }
        }
    };
}

impl_get_bits!(u8);
impl_get_bits!(u16);
impl_get_bits!(u32);
impl_get_bits!(usize);

#[test]
fn get_bits() {
    let source: u32 = 0b1101_0011_1010;
    assert_eq!(source.get_bits(Offset(0), Length(4)), 0b1010);
    assert_eq!(source.get_bits(Offset(4), Length(4)), 0b0011);
    assert_eq!(source.get_bits(Offset(8), Length(4)), 0b1101);
}
