use crate::calc::formato_to_string;

use crate::{FormatOptions, Formato};

macro_rules! formato {
    ($type:ty) => {
        impl Formato for $type {
            fn formato(&self, format: &str) -> String {
                self.formato_ops(format, &FormatOptions::default())
            }

            fn formato_ops(&self, format: &str, ops: &FormatOptions) -> String {
                formato_to_string(*self, format, ops)
            }
        }
    };
}

formato!(isize);
formato!(i128);
formato!(i64);
formato!(i32);
formato!(i16);
formato!(i8);

formato!(usize);
formato!(u128);
formato!(u64);
formato!(u32);
formato!(u16);
formato!(u8);

formato!(f32);
formato!(f64);
