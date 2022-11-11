use std::fmt::Display;

pub struct NumberComponents {
    pub sign: Sign,
    pub pre: String,
    pub post: String,
}

#[derive(PartialEq, Eq, Clone)]
pub enum Sign {
    Positive,
    Negative,
    Zero,
}

pub fn get_number_components<T: Display + PartialOrd + PartialEq + Default + Copy>(
    num: &T,
) -> NumberComponents {
    let (pre, post) = pre_post(num);
    NumberComponents {
        sign: sign(num),
        pre,
        post,
    }
}
fn sign<T: PartialOrd + PartialEq + Default>(val: &T) -> Sign {
    if val == &T::default() {
        Sign::Zero
    } else if val < &T::default() {
        Sign::Negative
    } else {
        Sign::Positive
    }
}

//get int and decimal parts
fn pre_post<T: Display>(number: T) -> (String, String) {
    let prepost = format!("{number}").replace('-', "");
    let mut prepost = prepost.split('.').take(2);

    let pre = prepost.next().unwrap_or_default().to_string();
    let post = prepost.next().unwrap_or_default().to_string();
    (pre, post)
}
