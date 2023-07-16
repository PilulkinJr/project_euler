pub fn multiples(n: u64) -> u64 {
    let mut result: u64 = 0;
    for i in 1..n {
        if i % 3 == 0 {
            result += i;
        } else if i % 5 == 0 {
            result += i;
        }
    }
    result
}

fn is_divisible_by_range(number: u64, start: u64, end: u64) -> bool {
    for i in (start..=end).rev() {
        if number % i != 0 {
            return false;
        }
    }
    true
}


pub fn smallest_multiple(start: u64, end: u64) -> u64 {
    let number: u64 = end;
    for number in end.. {
        if is_divisible_by_range(number, start, end) {
            return number;
        }
    }
    return number;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_multiples() {
        assert_eq!(multiples(10), 23);
        println!("Problem 1 answer is {}", multiples(1000));
    }
    #[test]
    fn test_smallest_multiple() {
        assert_eq!(smallest_multiple(1, 10), 2520);
        println!("Problem 5 answer is {}", smallest_multiple(1, 20));
    }

}