pub fn fibonacci(index: u16) -> u16 {
    match index {
        0 => 0,
        1 => 1,
        i => fibonacci(i - 1) + fibonacci(i - 2),
    }
}

#[cfg(test)]
mod fibonacci_tests {
    use crate::fibonacci::*;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(0, 0)]
    #[test_case(1, 1)]
    #[test_case(2, 1)]
    #[test_case(3, 2)]
    #[test_case(4, 3)]
    #[test_case(5, 5)]
    #[test_case(6, 8)]
    #[test_case(7, 13)]
    #[test_case(12, 144)]
    fn fibonacci_number_is_the_sum_of_the_two_preceding_ones(index: u16, fibonacci_number: u16) {
        assert_eq!(fibonacci_number, fibonacci(index));
    }
}
