use crate::{
    comps::{NumberComponents, Sign},
    FormatOptions,
};

pub fn calc_format(comps: NumberComponents, mut format: &str, ops: &FormatOptions) -> String {
    if format.is_empty() {
        format = "#";
    }
    //split into +;-;0
    let format = get_pos_neg_zero_format(format);
    let typ = get_format_type(&format);

    match typ {
        FormatType::Number { prec } => custom(
            (
                &format!("#,##0{}", decimal_format_with_prec(prec)),
                None,
                None,
            ),
            comps,
            ops,
        ),
        FormatType::Fixed { prec } => custom(
            (&format!("0{}", decimal_format_with_prec(prec)), None, None),
            comps,
            ops,
        ),
        FormatType::Custom => custom(format, comps, ops),
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

fn custom(
    format: (&str, Option<&str>, Option<&str>),
    stats: NumberComponents,
    ops: &FormatOptions,
) -> String {
    let must_add_neg = stats.sign == Sign::Negative && format.1.is_none();

    let format = match (&stats.sign, format.0, format.1, format.2) {
        (Sign::Negative, _, Some(neg), _) => neg,
        (Sign::Zero, _, _, Some(zero)) => zero,
        _ => format.0,
    };

    let (format_int, format_decimal) = if let Some(a) = format.split_once('.') {
        (a.0, a.1)
    } else {
        (format, "")
    };

    let mut int_result = custom_int_part(format_int, &stats, ops);
    let (decimal_result, has_carry) = custom_decimal_part(format_decimal, stats);

    //round if necessary
    if has_carry {
        carry_the_one(&mut int_result, true);
    }

    let mut result = String::new();
    result.push_str(&int_result);
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
/// it will remember thousand separators, and repeat the last pattern (e.g. every 3 or 2)
/// it will output all other characters in format
fn custom_int_part(formatint: &str, comps: &NumberComponents, ops: &FormatOptions) -> String {
    let mut result = String::new();
    let mut formatpre = formatint.chars().rev();
    //format pre (start from end)
    let mut last_f_ch = '\0';
    //only #or0
    let mut thousands = 0;
    let mut thousands_start = 0;
    let mut other_buffer = vec![];
    //hold other chars. we only output at end, or if there is another placeholder
    for (counter, ch) in comps.pre.chars().rev().enumerate() {
        for chf in formatpre.by_ref() {
            //check if thousands separator
            if chf == ',' {
                thousands = counter - thousands_start;
                thousands_start = counter;
                //must output buffer because we have ,
                for ch in &other_buffer {
                    result.insert(0, *ch);
                }
                other_buffer.clear();

                result.insert_str(0, &ops.thousands);
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
        if thousands > 0
            && (counter != thousands_start)
            && ((counter - thousands_start) % thousands) == 0
        {
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
            other => result.insert(0, other),
        }
    }
    result
}

///Output number if there is one and pattern is # or 0
/// Output 0 if there is no trailing numbers
/// it will output all other characters in format
fn custom_decimal_part(formatdecimal: &str, stats: NumberComponents) -> (String, bool) {
    let mut decimal_result = String::new();
    let mut formatpost = formatdecimal.chars();
    let mut last_f_ch;
    let mut next_char = None;
    for ch in stats.post.chars() {
        loop {
            if let Some(chf) = formatpost.next() {
                if is_placeholder(chf) {
                    last_f_ch = chf;
                    break;
                } else {
                    decimal_result.push(chf);
                }
            } else {
                next_char = Some(ch);
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

    //do rounding if necessary
    let mut has_carry = false;
    if let Some(after) = next_char {
        if (after as u8) - b'0' >= 5 {
            has_carry = carry_the_one(&mut decimal_result, false);
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
            other => decimal_result.push(other),
        }
    }
    (decimal_result, has_carry)
}

///rounds if necessary
/// TODO: any way we can do this smarter?
/// We are assuming that each '0'-'9' is from the number,
///  which may not be true, as it may come from the format (unlikely but possible)
/// must insert = true for int part as we may need to add 1 to start, false for decimal part
fn carry_the_one(input: &mut String, must_insert: bool) -> bool {
    //we start at end, and add to each char which is a '0'-'9' until no more carry

    let mut result = String::with_capacity(input.len());
    let chars = input.chars().rev();
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
    if has_carry && must_insert {
        result.insert(result.len() - last_carry_pos, '1');
        has_carry = false;
    }
    *input = result;
    has_carry
}

enum FormatType {
    Fixed { prec: u8 },  //F default 2
    Number { prec: u8 }, //N default 2
    Custom,
}

fn get_format_type(format: &(&str, Option<&str>, Option<&str>)) -> FormatType {
    //if we have negative and/or zero then it is custom
    if format.1.is_some() || format.2.is_some() {
        return FormatType::Custom;
    }

    let format = format.0;

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

fn get_pos_neg_zero_format(format: &str) -> (&str, Option<&str>, Option<&str>) {
    let spl: Vec<&str> = format.split(';').collect();

    match spl.len() {
        1 => (format, None, None),
        2 => (spl[0], Some(spl[1]), None),
        _ => (spl[0], Some(spl[1]), Some(spl[2])),
    }
}
