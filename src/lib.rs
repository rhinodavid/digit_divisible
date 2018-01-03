pub fn run(x: u32) -> Result<bool, &'static str> {
    match x == 0 {
        false => Ok(make_digits(x).iter().all(|d| is_evenly_divisible(x, *d))),
        true => Err("This doesn't work with zero!"),
    }
}

fn is_evenly_divisible(quotient: u32, divisor: u32) -> bool {
    match divisor == 0 {
        false => quotient % divisor == 0,
        true => false,
    }
}

fn make_digits(mut x: u32) -> Vec<u32> {
    let mut result = vec![];
    if x == 0 {
        return result;
    }
    while x > 10 {
        result.push(x % 10);
        x = x / 10;
    }
    result.push(x);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_digits() {
        assert_eq!(
            make_digits(123).sort_unstable(),
            vec![1, 2, 3].sort_unstable()
        );
    }

    #[test]
    fn test_is_evenly_divisible() {
        assert_eq!(is_evenly_divisible(10, 2), true);
        assert_eq!(is_evenly_divisible(11, 2), false);
        assert_eq!(is_evenly_divisible(111, 1), true);
        assert_eq!(is_evenly_divisible(20, 0), false);
    }

    #[test]
    fn test_run() {
        assert_eq!(run(244), Ok(true));
        assert_eq!(run(211), Ok(false));
        assert_eq!(run(0), Err("This doesn't work with zero!"));
    }
}
