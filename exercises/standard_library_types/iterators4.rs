// iterators4.rs

// I AM NOT DONE

pub fn factorial(num: u64) -> u64 {
    match num {
        0 => 0,
        1 => 1,
        _ => num * factorial(num - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
