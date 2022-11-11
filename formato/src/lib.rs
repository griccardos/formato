mod calc;
mod comps;
mod impls;

/// Trait for number types to return formatted string
pub trait Formato {
    /// Convert number to string.
    /// This can be using build in format, or a custom format
    ///
    /// ## Built in formats
    /// where 'd' is the optional number of decimals. when left out, it defaults to 2
    /// - "Fd": format with fixed number of decimal places
    /// - "Nd": format with thousand separators and fixed number of decimal places
    /// ```rust
    /// use formato::*;
    /// let num:f64 = 1234.1234;
    ///  assert_eq!("1,234.12", num.formato("N"));
    ///  assert_eq!("1,234.1", num.formato("N1"));
    ///  assert_eq!("1234.12", num.formato("F"));
    ///  assert_eq!("1234.1", num.formato("F1"));
    /// ```
    ///
    /// ## Custom symbols
    /// These are used together to build the format you require
    /// - "0" replace with digit if there is one otherwise 0
    /// - "#" replace with digit if there is one else nothing
    /// - "." sets the decimal position (only the 1st found)
    /// - "," sets the grouping location (repeats the last pattern found on int part. decimal part it acts as normal character)
    /// - ";" optionally separate positive, negative, zero formats. e.g. 0;(0);-
    /// - all others characters are output as is
    ///
    /// ```rust
    /// use formato::*;
    /// let num:u32 = 1234;
    ///  assert_eq!("$ 1,234.00", num.formato("$ #,###.00"));
    ///  assert_eq!("1234.00", num.formato("0.00"));
    /// ```
    fn formato(&self, format: &str) -> String;
    /// Convert number to string and change thousands and/or decimal separator
    /// ```rust
    /// use formato::{FormatOptions, Formato};
    ///
    /// let ops = FormatOptions::default()
    /// .with_thousands(" ")
    /// .with_decimal(",");
    ///
    /// assert_eq!("1 234,00", 1234.formato_ops("#,###.00",&ops));
    /// ```
    fn formato_ops(&self, format: &str, ops: &FormatOptions) -> String;
}

pub struct FormatOptions {
    pub thousands: String,
    pub decimal: String,
}
impl FormatOptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_thousands(mut self, thousands: &str) -> Self {
        self.thousands = thousands.to_string();
        self
    }
    pub fn with_decimal(mut self, decimal: &str) -> Self {
        self.decimal = decimal.to_string();
        self
    }
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            thousands: ",".to_string(),
            decimal: ".".to_string(),
        }
    }
}

#[cfg(test)]
mod tests;
