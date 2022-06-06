use num_traits::int::PrimInt;
use num_traits::AsPrimitive;

pub trait Input: PrimInt + AsPrimitive<f32> + Clone + Copy {}

impl Input for u8 {}
impl Input for u32 {}
