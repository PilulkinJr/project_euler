fn is_palindrome(number: u64) -> bool {
    let s: String = number.to_string();
    let s_len = s.len();

    if s_len < 2 {
        return true;
    }

    for i in 0..s_len / 2 {
        let left: char = s.as_bytes()[i] as char;
        let right: char = s.as_bytes()[s_len - 1 - i] as char;
        if left != right {
            return false;
        }
    }
    true
}

pub fn largest_palindrome(n_digits: u32) -> u64 {
    let mut result: u64 = 0;
    let left_boundary: u64 = 10_u64.pow(n_digits - 1);
    let right_boundary: u64 = 10_u64.pow(n_digits);

    for i in (left_boundary..right_boundary).rev() {
        for j in (left_boundary..i + 1).rev() {
            let number: u64 = i * j;
            if is_palindrome(number) && number > result {
                result = number;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::palindromes::{is_palindrome, largest_palindrome};


    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(9009))
    }

    #[test]
    fn test_largest_palindrome_2() {
        assert_eq!(largest_palindrome(2), 9009)
    }

}