pub trait GetBits: Sized {
    fn get_bits(self: Self, offset: Offset<Self>, length: Length<Self>) -> Self;
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
        }
    };
}

impl_get_bits!(u8);
impl_get_bits!(u16);
impl_get_bits!(u32);
impl_get_bits!(usize);
