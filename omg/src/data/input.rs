use num_traits::{AsPrimitive, Num};

pub trait Input: Num + AsPrimitive<f32> + PartialOrd + Clone + Copy {}

impl Input for u8 {}
impl Input for u32 {}
impl Input for f32 {}
