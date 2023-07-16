pub fn isqrt(n: u64) -> u64 {
    (n as f64).sqrt() as u64
}

// pub fn is_prime(n: u64) -> bool {

//     if n == 1 {
//         return false;
//     }

//     if n == 2 {
//         return true;
//     }

//     let upper_limit: u64 = isqrt(n);
//     let mut counter: u64 = 0;

//     for i in 2..=upper_limit {
//         counter += 1;
//         if n % i == 0 {
//             return false;
//         }
//     }

//     println!("is_prime({}) {} iterations limit = {}", n, counter, upper_limit);

//     true
// }

// pub fn is_prime_alt(number: u64) -> bool {

//     if number < 2 {
//         return false;
//     }

//     let mut divisor: u64 = 2;
//     let mut quotient: u64 = number.div_euclid(divisor);
//     let mut remainder: u64 = number % 2;

//     while divisor <= quotient {
//         if remainder == 0 {
//             return false;
//         } else {
//             divisor += 1;
//             quotient = number.div_euclid(divisor);
//             remainder = number % divisor;
//         }
//     }

//     true

// }

pub fn is_prime(n: u64) -> bool {

    if n <= 3 {
        return n > 1;
    }

    if (n % 2 == 0) || (n % 3 == 0) {
        return false;
    }

    let mut i: u64 = 5;

    while i * i <= n {
        if (n % i == 0) || (n % (i + 2) == 0) {
            return false;
        }
        i += 6;
    }


    true
}


// pub fn prime_factors(number: u64) -> Vec<u64> {
//     let mut result: Vec<u64> = Vec::new();
//     if number < 2 {
//         return result;
//     }
//     let upper_limit: u64 = (number as f64).sqrt() as u64 + 1;
//     for i in 2..upper_limit {
//         if is_prime(i) && number % i == 0 {
//             result.push(i)
//         }
//     }
//     result
// }

pub fn largest_prime_factor(n: u64) -> u64 {

    if n < 2 {
        return 0;
    }

    if is_prime(n) {
        return n;
    }

    let limit: u64 = isqrt(n) + 1;

    for i in (2..=limit).rev() {
        if n % i != 0 {
            continue;
        }
        if !is_prime(i) {
            continue;
        }
        return i;
    }

    2
}


pub fn nth_prime(n: u64) -> u64 {

    if n == 1 {
        return 2;
    }

    if n == 2 {
        return 3;
    }

    let mut counter: u64 = 2;
    let mut i: u64 = 5;
    let mut result: u64 = 5;

    while counter < n {
        if is_prime(i) {
            counter += 1;
            result = i;
        }
        if counter == n { return result; }
        if is_prime(i + 2) {
            counter += 1;
            result = i + 2;
        }
        i += 6;
    }

    result
}

pub fn sum_of_primes_below(limit: u64) -> u64 {
    let mut result: u64 = 0;
    for n in 2..limit {
        if is_prime(n) {
            result += n;
        }
    }
    result
}

pub fn count_divisors(number: u64) -> u64 {
    if number == 1 {
        return 1;
    }

    let mut counter: u64 = 0;
    let mut divisor: u64 = 1;
    let mut quotient: u64 = number / 1;

    while divisor <= quotient {
        if number % divisor == 0 {
            if divisor == quotient {
                counter += 1;
            } else {
                counter += 2;
            }
        }
        divisor += 1;
        quotient = number / divisor;
    }

    counter
}

pub fn sum_of_proper_divisors(n: u64) -> u64 {
    if n == 1 { return 0; }
    if n == 2 { return 1; }

    let mut sum: u64 = 1;
    let mut left: u64 = 2;
    let mut right: u64 = n / 2;

    while left < right {
        if n % left == 0 {
            sum += left + right;
        }
        left += 1;
        right = n / left;
    }

    sum
}

pub fn amicable_numbers(limit: u64) -> u64 {
    let mut sum: u64 = 0;

    for i in 2..limit {
        let tmp = sum_of_proper_divisors(i);
        if tmp >= limit { continue; }
        if tmp == i { continue; }
        if sum_of_proper_divisors(tmp) == i { sum += i; }
    }

    sum
}

pub fn highly_divisible_triangle_number(threshold: u64) -> u64 {
    let mut triangle_number: u64 = 0;
    
    for n in 1.. {
        triangle_number += n;
        if count_divisors(triangle_number) > threshold {
            return triangle_number;
        }
    }
    triangle_number
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_largest_prime_factor() {
        assert_eq!(largest_prime_factor(5), 5);
    }

    #[test]
    fn test_nth_prime() {
        assert_eq!(nth_prime(6), 13);
    }

    #[test]
    fn test_sum_of_primes_below() {
        assert_eq!(sum_of_primes_below(10), 17);
    }

    #[test]
    fn test_count_divisors() {
        assert_eq!(count_divisors(28), 6);
    }

    #[test]
    fn test_highly_divisible_triangle_number() {
        assert_eq!(highly_divisible_triangle_number(5), 28);
    }

    #[test]
    fn test_sum_of_proper_divisors() {
        assert_eq!(sum_of_proper_divisors(220), 284);
        assert_eq!(sum_of_proper_divisors(284), 220);
    }

}