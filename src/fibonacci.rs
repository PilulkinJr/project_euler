fn fibonacci(n: u64) -> u64 {
    let mut a: u64 = 1;
    let mut b: u64 = 2;

    if n <= 0 {
        return 0;
    }

    if n == 1 {
        a
    } else if n == 2 {
        b
    } else {
        for _ in 3..=n {
            (a, b) = (b, a + b);
        }
        b
    }
}

pub fn sum_of_even_fibonacci(upper_bound: u64) -> u64 {
    let mut result = 0;
    for i in 1.. {
        let f = fibonacci(i);
        if f >= upper_bound {
            return result;
        }
        if f % 2 == 0 {
            result += f;
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_even_fibonacci() {
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 2);
        assert_eq!(fibonacci(3), 3);
        assert_eq!(fibonacci(10), 89);
        println!("Problem 2 answer is {}", sum_of_even_fibonacci(400000));
    }

}