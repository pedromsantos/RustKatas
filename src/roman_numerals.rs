const ARABIC_NUMBERS_TO_ROMAN_NUMERALS: [(u16, &'static str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

pub fn to_roman_numeral(number: u16) -> String {
    for (arabic_number, roman_numeral) in ARABIC_NUMBERS_TO_ROMAN_NUMERALS.iter() {
        if number >= *arabic_number {
            return (*roman_numeral).to_string() + &to_roman_numeral(number - arabic_number);
        }
    }

    "".to_string()
}

#[cfg(test)]
mod roman_numerals_tests {
    use crate::roman_numerals::*;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(1, "I")]
    #[test_case(2, "II")]
    #[test_case(3, "III")]
    #[test_case(4, "IV")]
    #[test_case(5, "V")]
    #[test_case(6, "VI")]
    #[test_case(7, "VII")]
    #[test_case(8, "VIII")]
    #[test_case(9, "IX")]
    #[test_case(10, "X")]
    #[test_case(11, "XI")]
    #[test_case(39, "XXXIX")]
    #[test_case(40, "XL")]
    #[test_case(44, "XLIV")]
    #[test_case(50, "L")]
    #[test_case(60, "LX")]
    #[test_case(70, "LXX")]
    #[test_case(89, "LXXXIX")]
    #[test_case(90, "XC")]
    #[test_case(100, "C")]
    #[test_case(400, "CD")]
    #[test_case(500, "D")]
    #[test_case(900, "CM")]
    #[test_case(1000, "M")]
    #[test_case(846, "DCCCXLVI")]
    #[test_case(1999, "MCMXCIX")]
    #[test_case(2008, "MMVIII")]
    fn roman_numeral_of_number_is(number: u16, roman_numeral: &'static str) {
        assert_eq!(roman_numeral, to_roman_numeral(number));
    }
}
