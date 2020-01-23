pub mod fizz_buzz {
    use std::borrow::Cow;

    pub fn fizz_buzzer(number: i8) -> Cow<'static, str> {
        match (number % 3, number % 5) {
            (0, 0) => "fizzbuzz".into(),
            (0, _) => "fizz".into(),
            (_, 0) => "buzz".into(),
            (_, _) => number.to_string().into(),
        }
    }
}

pub mod leap_year {
    pub fn is_leap(year: i16) -> bool {
        match (year % 400, year % 4, year % 100) {
            (0, _, _) => true,
            (_, 0, r) => r != 0,
            _ => false
        }
    }
}

pub mod fibonacci_sequence {
    pub fn fibonacci(index: i16) -> i16 {
        match index {
            0 => 0,
            1 => 1,
            i => fibonacci(i -1) + fibonacci(i - 2)
        }
    }
}

#[cfg(test)]
mod fizz_buzzer_tests {
    use super::fizz_buzz::*;
    use test_case::test_case;

    #[test_case(1, "1")]
    #[test_case(2, "2")]
    #[test_case(4, "4")]
    fn convert_non_multiples_of_three_or_and_five_to_text(number: i8, expected: &'static str) {
        assert_eq!(expected, fizz_buzzer(number));
    }

    #[test_case(3)]
    #[test_case(6)]
    #[test_case(9)]
    fn convert_multiples_of_three_to_fizz(number: i8) {
        assert_eq!("fizz", fizz_buzzer(number));
    }

    #[test_case(5)]
    #[test_case(10)]
    #[test_case(20)]
    fn convert_multiples_of_five_to_buzz(number: i8) {
        assert_eq!("buzz", fizz_buzzer(number));
    }

    #[test_case(15)]
    #[test_case(30)]
    #[test_case(45)]
    fn convert_multiples_of_three_and_five_to_fizzbuzz(number: i8) {
        assert_eq!("fizzbuzz", fizz_buzzer(number));
    }
}

#[cfg(test)]
mod leap_year_tests {
    use super::leap_year::*;
    use test_case::test_case;

    #[test_case(1970)]
    #[test_case(2030)]
    #[test_case(1990)]
    fn years_not_divisible_by_four_are_not_leap_years(year: i16) {
        assert_eq!(false, is_leap(year));
    }

    #[test_case(1700)]
    #[test_case(1900)]
    #[test_case(2100)]
    fn years_divisible_by_one_hundred_are_not_leap_years(year: i16) {
        assert_eq!(false, is_leap(year));
    }

    #[test_case(1600)]
    #[test_case(2000)]
    #[test_case(2400)]
    fn years_divisible_by_four_hundred_are_leap_years(year: i16) {
        assert_eq!(true, is_leap(year));
    }
    
    #[test_case(1804)]
    #[test_case(1808)]
    #[test_case(1812)]
    #[test_case(1816)]
    #[test_case(1820)]
    #[test_case(1824)]
    #[test_case(1828)]
    #[test_case(1832)]
    #[test_case(1836)]
    #[test_case(1840)]
    #[test_case(1844)]
    #[test_case(1848)]
    #[test_case(1852)]
    #[test_case(1856)]
    #[test_case(1860)]
    #[test_case(1864)]
    #[test_case(1868)]
    #[test_case(1872)]
    #[test_case(1876)]
    #[test_case(1880)]
    #[test_case(1884)]
    #[test_case(1888)]
    #[test_case(1892)]
    #[test_case(1896)]
    #[test_case(1904)]
    #[test_case(1908)]
    #[test_case(1912)]
    #[test_case(1916)]
    #[test_case(1920)]
    #[test_case(1924)]
    #[test_case(1928)]
    #[test_case(1932)]
    #[test_case(1936)]
    #[test_case(1940)]
    #[test_case(1944)]
    #[test_case(1948)]
    #[test_case(1952)]
    #[test_case(1956)]
    #[test_case(1960)]
    #[test_case(1964)]
    #[test_case(1968)]
    #[test_case(1972)]
    #[test_case(1976)]
    #[test_case(1980)]
    #[test_case(1984)]
    #[test_case(1988)]
    #[test_case(1992)]
    #[test_case(1996)]
    #[test_case(2004)]
    #[test_case(2008)]
    #[test_case(2012)]
    #[test_case(2016)]
    #[test_case(2020)]
    fn years_divisible_by_four_but_not_by_one_hundred_are_leap_years(year: i16) {
        assert_eq!(true, is_leap(year));
    }
}

#[cfg(test)]
mod fibonacci_tests {
    use super::fibonacci_sequence::*;
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
    fn fibonacci_number_for_index(index: i16, fibonacci_number: i16) {
        assert_eq!(fibonacci_number, fibonacci(index));
    }
}