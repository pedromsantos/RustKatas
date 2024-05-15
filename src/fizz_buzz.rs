pub fn fizz_buzzer(number: u8) -> String {
    match (number % 3, number % 5) {
        (0, 0) => String::from("fizzbuzz"),
        (0, _) => String::from("fizz"),
        (_, 0) => String::from("buzz"),
        (_, _) => number.to_string(),
    }
}

#[cfg(test)]
mod fizz_buzzer_tests {
    use crate::fizz_buzz::*;
    use pretty_assertions::assert_eq;
    use proptest::prelude::*;
    use test_case::test_case;

    #[test_case(1, "1")]
    #[test_case(2, "2")]
    #[test_case(4, "4")]
    fn convert_non_multiples_of_three_or_and_five_to_textual_representation(
        number: u8,
        expected: &'static str,
    ) {
        assert_eq!(expected, fizz_buzzer(number));
    }

    #[test_case(3)]
    #[test_case(6)]
    #[test_case(9)]
    fn convert_multiples_of_three_to_fizz(number: u8) {
        assert_eq!("fizz", fizz_buzzer(number));
    }

    #[test_case(5)]
    #[test_case(10)]
    #[test_case(20)]
    fn convert_multiples_of_five_to_buzz(number: u8) {
        assert_eq!("buzz", fizz_buzzer(number));
    }

    #[test_case(15)]
    #[test_case(30)]
    #[test_case(45)]
    fn convert_multiples_of_three_and_five_to_fizzbuzz(number: u8) {
        assert_eq!("fizzbuzz", fizz_buzzer(number));
    }

    proptest! {
            #[test]
            fn multiples_of_three_start_with_fizz(number in multiples_of_three(50)) {
                    assert_eq!(true, fizz_buzzer(number).starts_with("fizz"));
            }
    }

    proptest! {
    #[test]
            fn multiples_of_five_end_with_buzz(number in multiples_of_five(50)) {
                     assert_eq!(true, fizz_buzzer(number).ends_with("buzz"));
            }
    }

    proptest! {
            #[test]
            fn multiples_of_three_and_five_are_fizz_buzz(number in multiples_of_fifteen(50)) {
                    assert_eq!("fizzbuzz", fizz_buzzer(number));
            }
    }

    proptest! {
            #[test]
            fn multiples_of_neither_three_or_five_output_the_number(number: u8) {
                     prop_assume!(number % 3 != 0 && number % 5 != 0);

                     assert_eq!(format!("{}", number), fizz_buzzer(number))
            }
    }

    prop_compose! {
            fn multiples_of_three(max: u8)(base in 0..max) -> u8 { base * 3 }
    }

    prop_compose! {
        fn multiples_of_five(max: u8)(base in 0..max) -> u8 { base * 5 }
    }

    prop_compose! {
            fn multiples_of_fifteen(max: u8)(base in 0..max/5) -> u8 { base * 15 }
    }
}
