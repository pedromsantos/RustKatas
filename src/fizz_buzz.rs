pub fn fizz_buzzer(number: u8) -> String {
    match (number % 3, number % 5) {
        (0, 0) => String::from("fizzbuzz"),
        (0, _) => String::from("fizz"),
        (_, 0) => String::from("buzz"),
        (_, _) => number.to_string(),
    }
}

pub fn raindrops(n: u32) -> String {
    match (n % 3, n % 5, n % 7) {
        (0, 0, 0) => "PlingPlangPlong".to_string(),
        (0, 0, _) => "PlingPlang".to_string(),
        (0, _, 0) => "PlingPlong".to_string(),
        (_, 0, 0) => "PlangPlong".to_string(),
        (0, _, _) => "Pling".to_string(),
        (_, 0, _) => "Plang".to_string(),
        (_, _, 0) => "Plong".to_string(),
        _ => n.to_string(),
    }
}

#[cfg(test)]
mod fizz_buzzer_tests {
    use super::fizz_buzzer;
    use all_asserts::assert_true;
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
                    assert_true!(fizz_buzzer(number).starts_with("fizz"));
            }
    }

    proptest! {
            #[test]
            fn multiples_of_five_end_with_buzz(number in multiples_of_five(50)) {
                     assert_true!(fizz_buzzer(number).ends_with("buzz"));
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

#[cfg(test)]
mod raindrops_tests {
    use super::raindrops;

    #[test]
    fn the_sound_for_1_is_1() {
        let input = 1;
        let output = raindrops(input);
        let expected = "1";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_3_is_pling() {
        let input = 3;
        let output = raindrops(input);
        let expected = "Pling";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_5_is_plang() {
        let input = 5;
        let output = raindrops(input);
        let expected = "Plang";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_7_is_plong() {
        let input = 7;
        let output = raindrops(input);
        let expected = "Plong";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_6_is_pling_as_it_has_a_factor_3() {
        let input = 6;
        let output = raindrops(input);
        let expected = "Pling";
        assert_eq!(output, expected);
    }
    #[test]
    fn test_2_to_the_power_3_does_not_make_a_raindrop_sound_as_3_is_the_exponent_not_the_base() {
        let input = 8;
        let output = raindrops(input);
        let expected = "8";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_9_is_pling_as_it_has_a_factor_3() {
        let input = 9;
        let output = raindrops(input);
        let expected = "Pling";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_10_is_plang_as_it_has_a_factor_5() {
        let input = 10;
        let output = raindrops(input);
        let expected = "Plang";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_14_is_plong_as_it_has_a_factor_of_7() {
        let input = 14;
        let output = raindrops(input);
        let expected = "Plong";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_15_is_plingplang_as_it_has_factors_3_and_5() {
        let input = 15;
        let output = raindrops(input);
        let expected = "PlingPlang";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_21_is_plingplong_as_it_has_factors_3_and_7() {
        let input = 21;
        let output = raindrops(input);
        let expected = "PlingPlong";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_25_is_plang_as_it_has_a_factor_5() {
        let input = 25;
        let output = raindrops(input);
        let expected = "Plang";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_27_is_pling_as_it_has_a_factor_3() {
        let input = 27;
        let output = raindrops(input);
        let expected = "Pling";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_35_is_plangplong_as_it_has_factors_5_and_7() {
        let input = 35;
        let output = raindrops(input);
        let expected = "PlangPlong";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_49_is_plong_as_it_has_a_factor_7() {
        let input = 49;
        let output = raindrops(input);
        let expected = "Plong";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_52_is_52() {
        let input = 52;
        let output = raindrops(input);
        let expected = "52";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_105_is_plingplangplong_as_it_has_factors_3_5_and_7() {
        let input = 105;
        let output = raindrops(input);
        let expected = "PlingPlangPlong";
        assert_eq!(output, expected);
    }
    #[test]
    fn the_sound_for_3125_is_plang_as_it_has_a_factor_5() {
        let input = 3125;
        let output = raindrops(input);
        let expected = "Plang";
        assert_eq!(output, expected);
    }
}
