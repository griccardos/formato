pub trait FormatoDuration {
    fn formato(&self) -> String;
}
impl FormatoDuration for std::time::Duration {
    fn formato(&self) -> String {
        let mut time = self.as_millis();
        if time == 0 {
            return "0ms".to_string();
        }
        let mut str = String::new();
        for (ch, qu) in [("ms", 1000), ("s", 60), ("m", 60), ("h", 24)] {
            str.insert_str(0, &format!("{}{} ", next(&mut time, qu), ch));
            if time == 0 {
                break;
            }
        }
        if time != 0 {
            str.insert_str(0, &format!("{time}d "));
        }

        str.trim_end().to_string()
    }
}

fn next(num: &mut u128, quantum: u128) -> u128 {
    let high = *num / quantum;
    let low = *num - high * quantum;
    *num = high;
    println!("got {low} with left{high}");
    low
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::duration::FormatoDuration;

    #[test]
    fn def() {
        //1d 2h 3m 4s 5ms = 93784005
        let dur = Duration::from_millis(93784005);
        assert_eq!("1d 2h 3m 4s 5ms", dur.formato());

        //999ms

        let dur = Duration::from_millis(999);
        assert_eq!("999ms", dur.formato());
    }
}
