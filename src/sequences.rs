fn collatz_sequence_next(n: u64) -> u64 {
    if n % 2 == 0 {
        return n / 2;
    } else {
        return 3 * n + 1;
    }
}

fn collatz_sequence_len(start: u64) -> u64 {
    let mut number: u64 = start;
    let mut len: u64 = 1;
    while number != 1 {
        number = collatz_sequence_next(number);
        len += 1;
    }
    len
}

pub fn longest_collatz_sequence(threshold: u64) -> u64 {
    let mut longest_num: u64 = 0;
    let mut longest_len: u64 = 0;
    for n in 1..threshold {
        let len = collatz_sequence_len(n);
        if len > longest_len {
            longest_num = n;
            longest_len = len;
        }
    }
    longest_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_sequence_len() {
        assert_eq!(collatz_sequence_len(13), 10);
    }

}