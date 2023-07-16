pub fn add_string_numerals(first_string: &str, second_string: &str) -> String {
    let mut result_string: String = String::new();

    let mut longest: String;
    let mut shortest: String;

    if first_string.len() > second_string.len() {
        longest = String::from(first_string);
        shortest = String::from(second_string);
    } else {
        longest = String::from(second_string);
        shortest = String::from(first_string);
    }

    let pad = longest.len();

    shortest = format!("{:0>pad$}", shortest);

    let iter = longest.chars().rev().zip(shortest.chars().rev());
    let mut carry: u32 = 0;

    for (a, b) in iter {
        let a = a.to_digit(10_u32).unwrap();
        let b = b.to_digit(10_u32).unwrap();
        let mut c = a + b + carry;

        if c >= 10 {
            c -= 10;
            carry = 1;
        } else {
            carry = 0;
        }

        result_string = format!("{}{}", c, result_string);
        // println!("a = {} b = {} c = {} carry = {} result = {}", a, b, c, carry, result_string);
    }

    if carry == 1 {
        result_string = format!("{}{}", 1, result_string)
    }

    result_string
}

pub fn mul_string_numerals(first_string: &str, second_string: &str) -> String {
    let mut result_string: String = String::new();

    if first_string.len() == 0 {
        return String::from(second_string);
    }
    if second_string.len() == 0 {
        return String::from(first_string);
    }

    let mut longest: String;
    let mut shortest: String;

    if first_string.len() > second_string.len() {
        longest = String::from(first_string);
        shortest = String::from(second_string);
    } else {
        longest = String::from(second_string);
        shortest = String::from(first_string);
    }

    let pad = longest.len();
    shortest = format!("{:0>pad$}", shortest);

    let mut pad_a: usize = 0;
    let mut pad_b: usize = 0;
    for a in longest.chars().rev() {
        let a = a.to_digit(10_u32).unwrap();
        pad_b = 0;
        for b in shortest.chars().rev() {
            let b = b.to_digit(10_u32).unwrap();
            let total_pad = pad_a + pad_b;
            let c = a * b;
            if c != 0 {
                let c = c.to_string() + format!("{:0<total_pad$}", "").as_str();

                result_string = add_string_numerals(result_string.as_str(), &c);
            }
            // println!("a = {} b = {} c = {} result = {} total_pad = {}", a, b, c, result_string, total_pad);
            pad_b += 1;
        }
        pad_a += 1;
    }

    result_string
}

pub fn string_factorial(n: u32) -> String {
    let mut result = String::from("1");
    for i in 2..=n {
        let a = i.to_string();
        result = mul_string_numerals(&result, &a);
        // println!("{} {}", a, result)
    }

    result
}

pub fn sum_of_digits(s: &str) -> u32 {
    let string = String::from(s);
    let mut result: u32 = 0;

    for c in string.chars() {
        result += c.to_digit(10_u32).unwrap();
    }

    result
}

pub fn factorial_digit_sum(n: u32) -> u32 {
    let f = string_factorial(n);
    sum_of_digits(&f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(string_factorial(10), "3628800")
    }

    #[test]
    fn test_sum_of_digits() {
        assert_eq!(sum_of_digits("3628800"), 27)
    }

    #[test]
    fn test_factorial_digit_sum() {
        assert_eq!(factorial_digit_sum(10), 27)
    }
}
