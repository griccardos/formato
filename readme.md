# formato

Easily format numbers into string representation\
Works for  integers (u8-u128 and i8-i128) and floats (f32, f64)\
Allows you to specify location of thousands separator, number of decimals, different format for positive, negative and zero values. e.g.
- `1,000,000`
- `0012`
- `(4 234.56)`

Similar to numerical formatting in Excel and C#


## Simple examples
```rust
use formato::{Formato,FormatOptions};
assert_eq!("001", 1.formato("000"));
assert_eq!("1,234", 1234.formato("#,###"));
assert_eq!("1,234.00", 1234.formato("N2")); 
assert_eq!("(1,234)", (-1234).formato("#,##0 ;(#,##0);-"));

let ops=FormatOptions::default()
        .with_thousands(" ")
        .with_decimal(",");
assert_eq!("1 234,32", 1234.321.formato_ops("#,###.00",&ops));        
```
See below for more examples

## Roadmap and contributing
This is still a very early release so there may be bugs. If you find any bugs, please open an issue, or create a PR. 
There has been almost no performance tuning yet, first want to stabilise functionality.
That being said, if there are any obvious wins, you are welcome to create a PR.

#### Short term
- Clean up code
- Increase tests to find potential bugs
- Quick performance wins
- Stablise API

#### Long term
- Performance improvements

## Examples

### Placeholders
- "0" replace with digit if there is one otherwise 0
- "#" replace with digit if there is one else ignore
```rust
assert_eq!("001", 1.formato("000"));
assert_eq!("1", 1.formato("###"));
assert_eq!("01", 1.formato("#00"));
```


### Built in formats
where 'd' is the optional number of decimals. when left out, it defaults to 2
- "Fd": format with fixed number of decimal places
- "Nd": format with thousand separators and fixed number of decimal places
```rust
let num:f64 = 1234.1234;
 assert_eq!("1234.12", num.formato("F"));
 assert_eq!("1234.1", num.formato("F1"));
 assert_eq!("1,234.12", num.formato("N"));
 assert_eq!("1,234.1", num.formato("N1"));
 ```

### Rounding
where a decimal part is left out, it rounds if the next digit is 5 or above
```rust
assert_eq!("1,234.57", 1234.5678.formato("#,###.##"));
assert_eq!("$ 10,000.00", 9999.996.formato("$ #,###.##")); 
```

### Thousands separator
```rust
assert_eq!("1,234", 1234.formato("#,###"));

//pattern is repeated for more significant digits
assert_eq!("1,000,000", 1_000_000.formato("#,###"));

//Indian notation - left most pattern is repeated for more significant digits
assert_eq!("10,00,000", 1_000_000.formato("#,##,###"));
```

### Currency and other characters
```rust
//formato ignores characters other than #0,. and includes them as is
assert_eq!("$ 1,234.00", 1234.formato("$ #,###.00"));
assert_eq!("oh wow!❤1,234✔", 1234.formato("oh wow!❤#,###✔"));
```

### Custom thousands and decimals
"," sets the grouping location (repeats the last pattern found on int part. decimal part it acts as normal character)
```rust
let ops=FormatOptions::default()
        .with_thousands(" ")
        .with_decimal(",");
assert_eq!("1 234,00", 1234.formato_ops("#,###.00",&ops));
```

### Positive, Negative, Zero formats
";" optionally separate positive, negative, zero formats. e.g. 0;(0);-
```rust
let my_format = "#,###.00 ;(#,###.00);- ";
assert_eq!("1,234.57 ", 1234.567.formato(my_format));
assert_eq!("(1,234.57)", (-1234.567).formato(my_format));
assert_eq!("- ", 0.formato(my_format));
```