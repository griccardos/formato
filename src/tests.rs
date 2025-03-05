use crate::{FormatOptions, Formato};

#[test]
fn basics() {
    assert_eq!("1,234", 1234.formato("#,##0"));
    assert_eq!("1,234.00", 1234.formato("#,##0.00"));
    assert_eq!("65,535", (0xffff).formato("#,###"));
}

#[test]
fn zeros() {
    assert_eq!("$ 0.0", 0u8.formato("$ 0.0"));
    assert_eq!("$ 0.0", 0u16.formato("$ 0.0"));
    assert_eq!("$ 0.0", 0u32.formato("$ 0.0"));
    assert_eq!("$ 0.0", 0u64.formato("$ 0.0"));
    assert_eq!("$ 0.0", 0u128.formato("$ 0.0"));
    assert_eq!("$ 0.0", 0i8.formato("$ 0.0"));
    assert_eq!("$ 0.0", 0i16.formato("$ 0.0"));
    assert_eq!("$ 0.0", 0i32.formato("$ 0.0"));
    assert_eq!("$ 0.0", 0i64.formato("$ 0.0"));
    assert_eq!("$ 0.0", 0i128.formato("$ 0.0"));

    assert_eq!("$ 0.0", (-0i8).formato("$ 0.0"));
    assert_eq!("$ 0.0", (-0i16).formato("$ 0.0"));
    assert_eq!("$ 0.0", (-0i32).formato("$ 0.0"));
    assert_eq!("$ 0.0", (-0i64).formato("$ 0.0"));
    assert_eq!("$ 0.0", (-0i128).formato("$ 0.0"));

    assert_eq!("$ 0.0", (0f32).formato("$ 0.0"));
    assert_eq!("$ 0.0", (0f64).formato("$ 0.0"));
    assert_eq!("-$ 0.0", (-0f32).formato("$ 0.0"));
    assert_eq!("-$ 0.0", (-0f64).formato("$ 0.0"));
}

#[test]
fn custom_placeholders() {
    //test 0 and #
    assert_eq!("1", 1.formato("0"));
    assert_eq!("01", 1.formato("00"));
    assert_eq!("1", 1.formato("#"));
    assert_eq!("1", 1.formato("##"));
    assert_eq!("01", 1.formato("0#"));
    assert_eq!("1", 1.formato("#0"));
    assert_eq!("R0001", 1.formato("R#0#0#"));
    assert_eq!("001", 1.formato("0##"));
}

#[test]
fn custom_placeholders_neg() {
    //test 0 and # neg
    assert_eq!("-1", (-1).formato("0"));
    assert_eq!("-01", (-1).formato("00"));
    assert_eq!("-1", (-1).formato("#"));
    assert_eq!("-1", (-1).formato("##"));
    assert_eq!("-01", (-1).formato("0#"));
    assert_eq!("-1", (-1).formato("#0"));
    assert_eq!("-R0001", (-1).formato("R#0#0#"));
}

#[test]
fn custom_separators() {
    assert_eq!("0,001", 1.formato("0,000"));
    assert_eq!("001", 1.formato("#,000"));
    assert_eq!("1,000,000", 1_000_000.formato("#,###"));
    assert_eq!("1,000,000", 1_000_000.formato("#,##,###")); //no indian notation
}

#[test]
fn custom_other_characters() {
    assert_eq!("$ 1,234", 1234.formato("$ #,###"));
    assert_eq!("oh wow!❤1,234✔", 1234.formato("oh wow!❤#,###✔"));
    assert_eq!("1,00test0,000👍", 1000000.formato("#test#,###👍")); //if more placeholders, we place text within
    assert_eq!("test1,000👍", 1000.formato("#test#,###👍")); //if less placeholders, text is in order
    assert_eq!("test1,000,000👍", 1000000.formato("test#,###👍")); //if no more placeholders, we write text to full left
}

#[test]
fn custom_pos_neg_format() {
    assert_eq!("1,234 ", 1234.formato("#,##0 ;(#,##0)"));
    assert_eq!("(1,234)", (-1234).formato("#,##0 ;(#,##0)"));
}

#[test]
fn custom_pos_neg_zero_format() {
    assert_eq!("1,234 ", 1234.formato("#,##0 ;(#,##0);-"));
    assert_eq!("(1,234)", (-1234).formato("#,##0 ;(#,##0);-"));
    assert_eq!("- ", 0.formato("#,##0 ;(#,##0);- "));
    assert_eq!("-0-", 0.formato(";;-0-"));

    let my_format = "#,###.00 ;(#,###.00);- ";
    assert_eq!("1,234.57 ", 1234.567.formato(my_format));
    assert_eq!("(1,234.57)", (-1234.567).formato(my_format));
    assert_eq!("- ", 0.formato(my_format));
}

#[test]
fn int_types() {
    assert_eq!("R 1", 1u8.formato("R #"));
    assert_eq!("R 1", 1u16.formato("R #"));
    assert_eq!("R 1", 1u32.formato("R #"));
    assert_eq!("R 1", 1u64.formato("R #"));
    assert_eq!("R 1", 1u128.formato("R #"));

    assert_eq!("R 1", 1i8.formato("R #"));
    assert_eq!("R 1", 1i16.formato("R #"));
    assert_eq!("R 1", 1i32.formato("R #"));
    assert_eq!("R 1", 1i64.formato("R #"));
    assert_eq!("R 1", 1i128.formato("R #"));
}

#[test]
fn custom_options() {
    assert_eq!(
        "1'234",
        1234.formato_ops("#,###", &FormatOptions::default().with_thousands("'"))
    );
    assert_eq!(
        "1❤234",
        1234.formato_ops("#,###", &FormatOptions::default().with_thousands("❤"))
    );
    assert_eq!(
        "1 234,5678",
        1234.5678.formato_ops(
            "# ##0.####",
            &FormatOptions::default()
                .with_thousands(" ")
                .with_decimal(",")
        )
    );
}

#[test]
fn custom_decimals() {
    //test 0 and #
    assert_eq!("1,234.567,8", 1234.5678.formato("#,###.###,#"));
    assert_eq!("1,234.567,hello8", 1234.5678.formato("#,###.###,hello#"));
    assert_eq!("1,234.0", 1234.formato("#,##0.0#####"));
    assert_eq!("1,234.0000000", 1234.formato("#,##0.0#####0"));
    assert_eq!("1,234.0", 1234.formato("#,##0.0######"));
}

#[test]
fn inbuilt() {
    let ops = FormatOptions::default()
        .with_thousands(" ")
        .with_decimal(",");
    //number
    assert_eq!("1,234.56781", 1234.56781.formato("N5"));
    assert_eq!("1,234.5678", 1234.56781.formato("n4"));
    assert_eq!("1 234,56781", 1234.56781.formato_ops("N5", &ops));
    assert_eq!("1 234,5678", 1234.56781.formato_ops("n4", &ops));
    assert_eq!("1,234.00", 1234.formato("N2"));
    assert_eq!("1,234.44", 1234.444.formato("N2"));
    assert_eq!("1,234", 1234.444.formato("N0"));
    assert_eq!("1,234", 1234.formato("N0"));
    //fixed
    assert_eq!("1234.56781", 1234.56781.formato("F5"));
    assert_eq!("1234.5678", 1234.56781.formato("f4"));
    assert_eq!("1234,56781", 1234.56781.formato_ops("F5", &ops));
    assert_eq!("1234,5678", 1234.56781.formato_ops("f4", &ops));
    assert_eq!("1234.00", 1234.formato("F2"));
    assert_eq!("1234.44", 1234.444.formato("F2"));
}

#[test]
fn rounding() {
    assert_eq!("1,234.57", 1234.5678.formato("#,###.##"));
    assert_eq!("1,235", 1234.5678.formato("#,##0"));
    assert_eq!("1,234.57", 1234.56781.formato("N2"));
    assert_eq!("1,234.56", 1234.561.formato("N2")); //no
    assert_eq!("1,234.57", 1234.565.formato("N2")); //round
    assert_eq!("1,234.57", 1234.569.formato("N2")); //round
    assert_eq!("$ 9,999.99", 9999.991.formato("$ #,###.00")); //no
    assert_eq!("$ 10,000.00", 9999.996.formato("$ #,###.00")); //round

    assert_eq!("1.0", 0.999.formato("0.0#")); //round
    assert_eq!("0.91", 0.909.formato("0.0#")); //round
    assert_eq!("0.9", 0.909.formato("0.0")); //no round
    assert_eq!("0.9", 0.9.formato("0.0#")); //no round

    assert_eq!("1234.57", 1234.565.formato("0.00"));
    assert_eq!("-1234.57", (-1234.565).formato("0.00"));

    assert_eq!("1.0", 1.00001.formato("0.0#")); //# at end is inserted, only if not 0
    assert_eq!("1000.0", 1000.00001.formato("0.0"));
    assert_eq!("1000.0", 1000.0000.formato("0.0"));
}

#[test]
fn readme() {
    assert_eq!("001", 1.formato("000"));
    assert_eq!("1,234", 1234.formato("#,###"));
    assert_eq!("1,234,567", 1234567.formato("#,###")); //pattern is repeated for more significant digits
    assert_eq!("1,234.00", 1234.formato("N2"));
    assert_eq!("1234.000", 1234.formato("F3"));

    let ops = FormatOptions::default()
        .with_thousands(" ")
        .with_decimal(",");
    assert_eq!("1 234,32", 1234.321.formato_ops("#,###.00", &ops));

    assert_eq!("1,000,000", 1_000_000.formato("#,###"));

    //Currency
    assert_eq!("$ 1,234.00", 1234.formato("$ #,###.00"));

    //Other characters included
    assert_eq!("oh wow!❤1,234✔", 1234.formato("oh wow!❤#,###✔"));

    //Custom thousands and decimals
    let ops = FormatOptions::default()
        .with_thousands(" ")
        .with_decimal(",");
    assert_eq!("1 234,00", 1234.formato_ops("#,###.00", &ops));

    assert_eq!("001", 1.formato("000"));
    assert_eq!("1", 1.formato("###"));
    assert_eq!("01", 1.formato("#00"));
}

#[test]
fn blanks() {
    assert_eq!("", 1.formato(""));
    assert_eq!("", (1.1).formato(""));
    assert_eq!("3", 1.formato("3")); //no placeholders, so output text
    assert_eq!("K9", 1.formato("K9")); //incorrect builtin, so output text
    assert_eq!("N1111", (1.2).formato("N1111")); //incorrect builtin, so output text
}

#[test]
fn really_big() {
    assert_eq!(
        "300,000,000,000,000,000,000,000,000,000,000,000,000,000",
        3e41.formato("#,###")
    );
    assert_eq!(
        "0.00000000000000000001",
        0.000000000000000000011.formato("#.00000000000000000000")
    );

    assert_eq!(
        "0.00000000000000000001",
        0.000000000000000000011.formato("#.00000000000000000000")
    );
}

#[test]
fn extremes() {
    assert_eq!("127", i8::MAX.formato("#,###"));
    assert_eq!("-128", i8::MIN.formato("#,###"));
    assert_eq!("32,767", i16::MAX.formato("#,###"));
    assert_eq!("-32,768", i16::MIN.formato("#,###"));
    assert_eq!("2,147,483,647", i32::MAX.formato("#,###"));
    assert_eq!("-2,147,483,648", i32::MIN.formato("#,###"));
    assert_eq!("9,223,372,036,854,775,807", i64::MAX.formato("#,###"));
    assert_eq!("-9,223,372,036,854,775,808", i64::MIN.formato("#,###"));
    assert_eq!(
        "170,141,183,460,469,231,731,687,303,715,884,105,727",
        i128::MAX.formato("#,###")
    );
    assert_eq!(
        "-170,141,183,460,469,231,731,687,303,715,884,105,728",
        i128::MIN.formato("#,###")
    );

    assert_eq!(
        "340,282,350,000,000,000,000,000,000,000,000,000,000",
        f32::MAX.formato("#,###")
    );
    assert_eq!(
        "-340,282,350,000,000,000,000,000,000,000,000,000,000",
        f32::MIN.formato("#,###")
    );
    assert_eq!("179,769,313,486,231,570,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000", f64::MAX.formato("#,###"));
    assert_eq!("-179,769,313,486,231,570,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000", f64::MIN.formato("#,###"));

    assert_eq!("NaN", f64::NAN.formato("$ #,###.00"));
    assert_eq!("-inf", f64::NEG_INFINITY.formato("$ #,###.0"));
    assert_eq!("inf", f64::INFINITY.formato("$ #,###.0"));
}

#[test]
fn perc() {
    //0% or 0.00%
    assert_eq!("123400%", 1234f32.formato("0%"));
    assert_eq!("123400.00%", 1234f32.formato("0.00%"));
    assert_eq!("1200%", 12f32.formato(r#"0%;"-"0%;0%"#));
    assert_eq!("-1200%", (-12f32).formato(r#"0%;"-"0%;0%"#));
}

#[test]
fn csharp_tests() {
    assert_eq!("1,234.00", 1234f32.formato("#,0.00"));
    assert_eq!("001,234", 1234f32.formato("000,000"));
}

#[test]
///these differ from csharp
fn csharp_deviations() {
    //Console.WriteLine(9999.991.ToString("$ h#e,l#l#o#.!0!0"));
    //$ h9,el9l9o9.!9!9
    //why is the , before the el whereas we requested it between e and l
    //we rather ignore all commas as they are a symbol of thousands
    assert_eq!("$ h9el9l9o9.!9!9", 9999.991.formato("$ h#e,l#l#o#.!0!0")); //round
    assert_eq!("$ h10el0l0o0.!0!0", 9999.996.formato("$ h#e,l#l#o#.!0!0")); //round
}

#[test]
fn custom_numeric() {
    //https://learn.microsoft.com/en-us/dotnet/standard/base-types/custom-numeric-format-strings

    assert_eq!("1.2", 1.2f32.formato("#.##"));
    assert_eq!("123", 123f32.formato("#####"));
    assert_eq!("[12-34-56]", 123456f32.formato("[##-##-##]"));
    assert_eq!("1234567890", 1234567890.formato("#"));
    assert_eq!("(123) 456-7890", 1234567890.formato("(###) ###-####"));

    assert_eq!("1.20", 1.2f32.formato("0.00"));
    assert_eq!("01.20", 1.2f32.formato("00.00"));
    assert_eq!("8.6%", 0.086f32.formato("0.##%"));

    assert_eq!("1,234,567,890", 1234567890f64.formato("#,#"));
    assert_eq!("1,235", 1234567890f64.formato("#,##0,,"));
}

#[test]
fn thousands() {
    use super::calc::check_if_thousands;
    assert!(check_if_thousands("#,###"));
    assert!(check_if_thousands("0,0"));
    assert!(check_if_thousands("0hello,0"));
    assert!(!check_if_thousands("0,b0"));
    assert!(!check_if_thousands("0,❤0"));
}

#[test]
fn trailingcomma() {
    assert_eq!("1,235", 1234567890f64.formato("#,##0,,"));
    assert_eq!("1,234", 1234567890.formato("#,##0,,"));
    assert_eq!("1234568", 1234567890f64.formato("#,"));
}
