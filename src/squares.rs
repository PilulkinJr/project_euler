use std::str::FromStr;

pub fn sum_square_difference(limit: u64) -> u64 {
    let mut result: u64 = 0;
    for i in 1..=limit {
        for j in (i + 1)..=limit {
            result += i * j;
        }
    }
    result * 2
}

pub fn special_pythagorean_triplet(sum: u64) -> u64 {
    for a in 1..=sum {
        for b in (a + 1)..=(sum - a - 1) {
            let c = sum - (a + b);
            if c <= b {
                break;
            }
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    return 0;
}

fn string_times_two(s: String) -> String {
    let mut t: String = String::new();
    let mut memory: u32 = 0;

    for c in s.chars().rev() {

        let mut d = c.to_digit(10_u32).unwrap();
        d = d * 2 + memory;
        if d >= 10 {
            memory = 1;
            d -= 10;
        } else {
            memory = 0;
        }
        t = d.to_string() + &t;
    }

    if memory > 0 {
        t = memory.to_string() + &t;
    }

    t

}

fn power_of_2(power: u64) -> String {
    if power == 0 {
        return String::from("1");
    }
    if power == 1 {
        return String::from("2");
    }
    let mut result: String = String::from("2");
    for _n in 1..power {
        result = string_times_two(result);
    }
    result
}

fn sum_of_digits(s: String) -> u32 {
    let mut result: u32 = 0;
    for c in s.chars() {
        let d = c.to_digit(10_u32).unwrap();
        result += d;
    }
    result
}

pub fn power_digit_sum(power: u64) -> u32 {
    let s: String = power_of_2(power);
    sum_of_digits(s)
}

fn factorial(n: u64) -> u64 {
    let mut result: u64 = 1;
    for i in 1..=n {
        result *= i;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_square_difference() {
        assert_eq!(sum_square_difference(10), 2640)
    }

    #[test]
    fn test_special_pyth_triplet() {
        assert_eq!(special_pythagorean_triplet(12), 60)
    }

    #[test]
    fn test_string_times_two() {
        assert_eq!(string_times_two(String::from("64")), "128");
    }

    #[test]
    fn test_power_of_2() {
        assert_eq!(power_of_2(4), "16");
    }

    #[test]
    fn test_power_digit_sum() {
        assert_eq!(power_digit_sum(15), 26);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(10), 3628800);
    }

}