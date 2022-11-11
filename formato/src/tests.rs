use crate::{FormatOptions, Formato};

#[test]
fn basics() {
    assert_eq!("1,234", 1234.formato("#,##0"));
    assert_eq!("1,234.00", 1234.formato("#,##0.00"));
    assert_eq!("65,535", (0xffff).formato("#,###"));
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
    assert_eq!("10,00,000", 1_000_000.formato("#,##,###")); //indian notation. this is different to c#. c# =1,000,000 formato=10,00,000
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
    assert_eq!("$ 9,999.99", 9999.991.formato("$ #,###.##")); //no
    assert_eq!("$ 10,000.00", 9999.996.formato("$ #,###.##")); //round
    assert_eq!("$ h9e,l9l9o9.!9!9", 9999.991.formato("$ h#e,l#l#o#.!#!#")); //round
    assert_eq!("$ h10e,l0l0o0.!0!0", 9999.996.formato("$ h#e,l#l#o#.!#!#")); //round

    assert_eq!("1234.57", 1234.565.formato("0.00"));
    assert_eq!("-1234.57", (-1234.565).formato("0.00"));
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

    //Indian notation
    assert_eq!("10,00,000", 1_000_000.formato("#,##,###"));

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
