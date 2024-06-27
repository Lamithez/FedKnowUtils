use std::collections::HashMap;

// 高精度算法

#[allow(unused)]

fn u8_add(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let mut answer = Vec::new();
    let mut pointer = 0;
    let mut carry = 0;
    while pointer < a.len() || pointer < b.len() || carry != 0 {
        let sum =
            if pointer < a.len() { a[pointer] } else { 0 } +
                if pointer < b.len() { b[pointer] } else { 0 } + carry;
        answer.push(sum % 10);
        carry = sum / 10;
        pointer += 1;
    }
    answer
}

pub fn add(a: &str, b: &str) -> String {
    let a: Vec<u8> = a.bytes().map(|x| x - b'0').rev().collect();
    let b: Vec<u8> = b.bytes().map(|x| x - b'0').rev().collect();
    u8_add(a, b).into_iter().map(|i| (i + b'0') as char).rev().collect()
}


pub fn multiple(a: &str, b: &str) -> String {
    if a == "0" || b == "0" {
        return String::from("0");
    }
    let mut answer = vec![];
    for (index, x) in a.bytes().rev().enumerate().map(|(index, x)| (index, x - b'0')) {
        let mut temp = vec![0; index];
        let mut carry = 0;
        for y in b.bytes().rev().map(|x| x - b'0') {
            let mul = x * y + carry;
            carry = mul / 10;
            temp.push(mul % 10);
        }
        if carry != 0 { temp.push(carry) };
        answer = u8_add(answer, temp);
    }
    answer.iter().map(|x| (x + b'0') as char).rev().collect()
}


pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    let numerator = numerator as i64;
    let denominator = denominator as i64;
    if numerator % denominator == 0 {
        return (numerator / denominator).to_string();
    }
    let mut answer = String::new();
    if (numerator < 0) != (denominator < 0) {
        answer.push('-');
    }
    //整数部分
    let num = numerator.abs() as u64;
    let den = denominator.abs() as u64;

    answer = format!("{}{}.", answer, num / den);
    let mut frac_part = String::new();
    let mut reminder_index = HashMap::new();
    let mut index = 0;
    let mut reminder = num % den;
    while reminder != 0 && !reminder_index.contains_key(&reminder) {
        reminder_index.insert(reminder, index);
        reminder *= 10;
        frac_part += &(reminder / den).to_string();
        reminder %= den;
        index += 1;
    }
    if reminder != 0 {
        frac_part.insert(reminder_index[&reminder], '(');
        frac_part.push(')');
    }
    answer + &frac_part
}

#[cfg(test)]
mod test {
    use num_bigint::RandBigInt;
    use rand::thread_rng;

    use crate::math::precision::{add, multiple};

    #[test]
    fn precision_test() {
        let mut rng = thread_rng();
        for _ in 0..10 {
            let a = rng.gen_biguint(256);
            let b = rng.gen_biguint(256);
            let sum = &a + &b;
            let mul = &a * &b;
            assert_eq!(sum.to_string(), add(&a.to_string(), &b.to_string()));
            assert_eq!(mul.to_string(), multiple(&a.to_string(), &b.to_string()));
        }
    }
}