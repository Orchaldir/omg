use num_traits::{AsPrimitive, Num};
use std::hash::Hash;

pub trait Input: Num + AsPrimitive<f32> + PartialOrd + Clone + Copy {}

impl Input for u8 {}
impl Input for u32 {}
impl Input for i32 {}
impl Input for f32 {}

pub trait IntInput: Input + Hash + Eq {}

impl IntInput for u8 {}
impl IntInput for u32 {}
impl IntInput for i32 {}
