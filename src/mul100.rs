//we need to change value if there is %
pub trait Mul100 {
    fn mul100(&self) -> Self;
}

macro_rules! impl_mul100 {
    // Match for integer types (e.g., i32, isize, usize, etc.)
    ($($t:ty),*) => {
        $(
            impl Mul100 for $t {
                fn mul100(&self) -> Self {
                    // Multiply integer types by 100
                    self * 100
                }
            }
        )*
    };

    // Match for floating-point types (e.g., f32, f64)
    (floats: $($t:ty),*) => {
        $(
            impl Mul100 for $t {
                fn mul100(&self) -> Self {
                    // Multiply floating-point types by 100.0
                    self * 100.0
                }
            }
        )*
    };
}

// Implement Mul100 for integer types
impl_mul100!(i32, isize, usize, i8, u8, i16, u16, i64, u64, i128, u128, u32);

// Implement Mul100 for floating-point types
impl_mul100!(floats: f32, f64);
