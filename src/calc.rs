use std::fmt::Display;

use crate::{div1000::Div1000, mul100::Mul100, FormatOptions};

pub fn formato_to_string<T>(mut num: T, format: &str, ops: &FormatOptions) -> String
where
    T: Display + Mul100 + Div1000,
{
    let mut value = format!("{num}");

    //special cases dont have any formatting
    match value.as_str() {
        "NaN" => return "NaN".to_string(),
        "-inf" => return "-inf".to_string(),
        "inf" => return "inf".to_string(),
        _ => {}
    }

    let sign = sign(&value);
    if sign == Sign::Negative {
        value = value.replace('-', "");
    }

    let pos_neg_zero = get_pos_neg_zero_format(format);
    let must_add_neg = sign == Sign::Negative && pos_neg_zero.neg.is_none();
    let format_to_use = calc_format(pos_neg_zero, &sign);

    percentage_multiply(&mut num, &mut value, &sign, &format_to_use);

    trailing_comma_divide(&mut num, &mut value, &sign, &format_to_use);

    round(&mut value, &format_to_use);
    let comps = get_int_decimal_parts(&value);
    apply_format(&format_to_use, comps, ops, must_add_neg)
}

fn percentage_multiply<T: Display + Mul100>(
    num: &mut T,
    value: &mut String,
    sign: &Sign,
    format_to_use: &String,
) {
    //check for % and multiply by 100
    let count_percentages = format_to_use.chars().filter(|&x| x == '%').count();
    for _ in 0..count_percentages {
        *num = num.mul100();
    }
    if count_percentages > 0 {
        *value = format!("{num}");
        if sign == &Sign::Negative {
            *value = value.replace('-', "");
        }
    }
}

//count number of trailing commas without any placeholders after them
fn trailing_comma_divide<T>(num: &mut T, value: &mut String, sign: &Sign, format_to_use: &String)
where
    T: Display + Mul100 + Div1000,
{
    let mut comma_count = 0;
    for ch in format_to_use.chars().rev() {
        if ch == ',' {
            comma_count += 1
        } else if is_placeholder(ch) {
            break;
        }
    }

    if comma_count == 0 {
        return;
    }

    for _ in 0..comma_count {
        *num = num.div1000();
    }
    *value = format!("{num}");
    if sign == &Sign::Negative {
        *value = value.replace('-', "");
    }
}

struct NumberComponents {
    pub int: String,
    pub decimal: String,
}

#[derive(PartialEq, Eq, Clone)]
enum Sign {
    Positive,
    Negative,
    Zero,
}

struct PosNegZero<'a> {
    pub pos: &'a str,
    pub neg: Option<&'a str>,
    pub zero: Option<&'a str>,
}

fn get_int_decimal_parts(num: &str) -> NumberComponents {
    let mut int_dec = num.split('.').take(2);

    let int = int_dec.next().unwrap_or_default().to_string();
    let decimal = int_dec.next().unwrap_or_default().to_string();

    NumberComponents { int, decimal }
}
fn sign(val: &str) -> Sign {
    if val.starts_with('-') {
        Sign::Negative
    } else if val == "0" {
        Sign::Zero
    } else {
        Sign::Positive
    }
}

fn round(val: &mut String, format: &str) {
    let format_decimals = format
        .chars()
        .skip_while(|x| x != &'.')
        .skip_while(|x| x == &'.')
        .filter(|&x| x == '0' || x == '#')
        .count();
    let mut dec_point = 0;
    let actual_decimals = val
        .chars()
        .enumerate()
        .skip_while(|(i, x)| {
            dec_point = *i;
            x != &'.'
        })
        .skip_while(|(_, x)| x == &'.')
        .count();

    //no rounding required because too few decimals in actual
    if actual_decimals <= format_decimals {
        return;
    }
    let next_ch = val
        .chars()
        .nth(dec_point + format_decimals + 1)
        .expect("actual_decimals > format_decimals");

    //no rounding required as next char is <5
    if ('0'..'5').contains(&next_ch) {
        let mut new = (val[0..(dec_point + format_decimals + 1)]).to_string();
        remove_trailing_zeros(&mut new);
        *val = new;
        return;
    }

    let mut result = String::with_capacity(val.len());
    let chars = val
        .chars()
        .rev()
        .skip(val.len() - dec_point - format_decimals - 1);
    let mut has_carry = true;
    let mut last_carry_pos = 0;
    for (i, ch) in chars.enumerate() {
        if has_carry && ('0'..='9').contains(&ch) {
            let mut val = ch as u32 - '0' as u32;
            val += 1;
            if val == 10 {
                result.insert(0, '0');
                last_carry_pos = i + 1;
            } else {
                result.insert_str(0, &val.to_string());
                has_carry = false;
            }
        } else {
            result.insert(0, ch);
        }
    }
    if has_carry {
        result.insert(result.len() - last_carry_pos, '1');
    }

    if dec_point > 0 {
        remove_trailing_zeros(&mut result);
    }

    *val = result;
}

fn remove_trailing_zeros(result: &mut String) {
    let dec_point = result.find(|x| x == '.');
    if dec_point.is_none() {
        return;
    }
    let dec_point = dec_point.unwrap();
    let len = result.len();

    //remove trailing 0's (and optionally .)
    let chars = result.chars();

    let mut num_trailing_zeros = chars.rev().take_while(|x| x == &'0').count();

    //get rid of trailing
    if dec_point == len - num_trailing_zeros - 1 {
        num_trailing_zeros += 1;
    }

    if num_trailing_zeros > 0 {
        *result = result[0..result.len() - num_trailing_zeros].to_string();
    }
}

enum FormatType {
    Fixed { prec: u8 },  //F default 2
    Number { prec: u8 }, //N default 2
    Custom,
}

fn get_format_type(format: &PosNegZero) -> FormatType {
    //if we have negative and/or zero then it is custom
    if format.neg.is_some() || format.zero.is_some() {
        return FormatType::Custom;
    }

    let format = format.pos;

    if format.is_empty() {
        return FormatType::Custom;
    }

    if format.len() > 2 {
        return FormatType::Custom;
    }

    let mut chars = format.chars();
    let first = chars.next();
    let second = chars.next();
    let second_num: Option<u8> = if let Some(pre) = second {
        if ((pre as u32) < b'0' as u32) || ((pre as u32) > b'9' as u32) {
            None //not valid
        } else {
            Some(pre as u8 - b'0')
        }
    } else {
        Some(2) //default
    };

    match (first, second_num) {
        (Some('N') | Some('n'), Some(prec)) => FormatType::Number { prec },
        (Some('F') | Some('f'), Some(prec)) => FormatType::Fixed { prec },
        _ => FormatType::Custom,
    }
}

fn get_pos_neg_zero_format(format: &str) -> PosNegZero {
    let spl: Vec<&str> = format.split(';').collect();

    match spl.len() {
        1 => PosNegZero {
            pos: format,
            neg: None,
            zero: None,
        },
        2 => PosNegZero {
            pos: spl[0],
            neg: Some(spl[1]),
            zero: None,
        },
        _ => PosNegZero {
            pos: spl[0],
            neg: Some(spl[1]),
            zero: Some(spl[2]),
        },
    }
}

fn calc_format(format: PosNegZero, sign: &Sign) -> String {
    let typ = get_format_type(&format);
    let pos_neg_zero = match typ {
        FormatType::Number { prec } => (
            format!("#,##0{}", decimal_format_with_prec(prec)),
            None,
            None,
        ),
        FormatType::Fixed { prec } => (format!("0{}", decimal_format_with_prec(prec)), None, None),
        FormatType::Custom => (format.pos.to_string(), format.neg, format.zero),
    };
    match (sign, &pos_neg_zero.0, pos_neg_zero.1, pos_neg_zero.2) {
        (Sign::Negative, _, Some(neg), _) => neg.to_string(),
        (Sign::Zero, _, _, Some(zero)) => zero.to_string(),
        _ => pos_neg_zero.0,
    }
}

fn decimal_format_with_prec(prec: u8) -> String {
    if prec == 0 {
        "".to_string()
    } else {
        let format = "0".repeat(prec as usize);
        format!(".{format}",)
    }
}

fn is_placeholder(ch: char) -> bool {
    ch == '0' || ch == '#'
}

fn apply_format(
    format: &str,
    components: NumberComponents,
    ops: &FormatOptions,
    must_add_neg: bool,
) -> String {
    let (format_int, format_decimal) = if let Some(a) = format.split_once('.') {
        (a.0, a.1)
    } else {
        (format, "")
    };

    let int_result = apply_format_int_part(format_int, &components, ops);
    let decimal_result = apply_format_decimal(format_decimal, &components);

    let mut result = int_result;
    if !decimal_result.is_empty() {
        result.push_str(&ops.decimal);
        result.push_str(&decimal_result);
    }

    if must_add_neg {
        result.insert(0, '-');
    }

    result
}

///Output number if # or 0
///Dont need to repeat exact pattern for entire number, it will write all significant digits
/// it will output all other characters in format
fn apply_format_int_part(formatint: &str, comps: &NumberComponents, ops: &FormatOptions) -> String {
    let mut result = String::new();
    let mut formatpre = formatint.chars().rev();
    //format pre (start from end)
    let mut last_f_ch = '\0'; //only #or0

    //has_thousands is true if there is a  # or 0, then anything, then comma, then # or 0
    let has_thousands = check_if_thousands(formatint);
    let mut in_quotes = false;
    let mut other_buffer = vec![]; //hold other chars. we only output at end, or if there is another placeholder
    for (number_counter, ch) in comps.int.chars().rev().enumerate() {
        for chf in formatpre.by_ref() {
            if chf == '"' {
                in_quotes = !in_quotes;
            } else if in_quotes {
                other_buffer.push(chf);
            } else if chf == ',' {
                //ignore thosands
            } else if !is_placeholder(chf) {
                //fill in any non placeholders
                other_buffer.push(chf);
            } else {
                //output any existing buffer because there is a placeholder
                for ch in &other_buffer {
                    result.insert(0, *ch);
                }
                other_buffer.clear();
                last_f_ch = chf;
                break;
            }
        }
        if has_thousands && number_counter > 0 && number_counter % 3 == 0 {
            result.insert_str(0, &ops.thousands);
        }

        match last_f_ch {
            '0' | '#' => result.insert(0, ch),
            _ => {} //do nothing if \0
        }
    }
    //output remaining buffer
    for ch in &other_buffer {
        result.insert(0, *ch);
    }
    //we want to ignore leading hashes, so we remove
    //for the remaining we need to remove all # in front of 0 (ignoring other chars),
    //also remove some other special characters
    let mut no_leading_hash: Vec<char> = formatpre.rev().collect();
    let mut i = 0;
    for _ in 0..no_leading_hash.len() {
        if no_leading_hash[i] == '0' {
            break;
        }
        if no_leading_hash[i] == '#' || no_leading_hash[i] == ',' {
            no_leading_hash.remove(i);
        } else {
            i += 1;
        }
    }
    let formatpre = no_leading_hash.into_iter().rev();
    //continue with rest of format
    for ch in formatpre {
        match ch {
            '0' | '#' => result.insert(0, '0'),
            '"' => {} //ignore quotes for rest, it will be displayed as is
            other => result.insert(0, other),
        }
    }
    result
}
/// thousand separators if:
/// a placeholder sometime before, then anything, then comma, then a placeholder immediately after

pub(crate) fn check_if_thousands(formatint: &str) -> bool {
    let mut found_first = false;
    for (i, ch) in formatint.chars().enumerate() {
        if is_placeholder(ch) {
            found_first = true;
        }
        if ch == ',' && found_first {
            if i < formatint.len() - 1 {
                if let Some(next) = formatint.chars().nth(i + 1) {
                    if is_placeholder(next) {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        }
    }
    false
}

///Output number if there is one and pattern is # or 0
/// Output 0 if there is no trailing numbers
/// it will output all other characters in format
fn apply_format_decimal(formatdecimal: &str, comps: &NumberComponents) -> String {
    let mut decimal_result = String::new();

    let mut formatpost = formatdecimal.chars();
    let mut last_f_ch;
    for ch in comps.decimal.chars() {
        loop {
            if let Some(chf) = formatpost.next() {
                if is_placeholder(chf) {
                    last_f_ch = chf;
                    break;
                } else {
                    decimal_result.push(chf);
                }
            } else {
                last_f_ch = '\0';
                break; //no more left
            }
        }

        match last_f_ch {
            '\0' => break,
            '0' | '#' => decimal_result.push(ch),
            other => panic!("unknown char {other}"),
        }
    }

    //get rid of trailing hashes,
    //since we are at the end of the input, we only want to display required zeros
    let mut no_trailing_hash = formatpost.collect::<Vec<char>>();
    if !no_trailing_hash.is_empty() {
        let mut i = no_trailing_hash.len() - 1;
        loop {
            if no_trailing_hash[i] == '0' {
                break;
            }
            if no_trailing_hash[i] == '#' {
                no_trailing_hash.remove(i);
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
    for ch in no_trailing_hash {
        match ch {
            '0' | '#' => decimal_result.push('0'),
            '"' => {} //ignore quotes for rest, it will be displayed as is
            other => decimal_result.push(other),
        }
    }
    decimal_result
}
