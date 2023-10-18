pub fn is_leap(year: u16) -> bool {
    match (year % 400, year % 4, year % 100) {
        (0, _, _) => true,
        (_, 0, r) => r != 0,
        _ => false,
    }
}

#[cfg(test)]
mod leap_year_tests {
    use crate::leap_year::*;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(1970)]
    #[test_case(2030)]
    #[test_case(1990)]
    fn years_not_divisible_by_four_are_not_leap_years(year: u16) {
        assert_eq!(false, is_leap(year));
    }

    #[test_case(1700)]
    #[test_case(1900)]
    #[test_case(2100)]
    fn years_divisible_by_one_hundred_are_not_leap_years(year: u16) {
        assert_eq!(false, is_leap(year));
    }

    #[test_case(1600)]
    #[test_case(2000)]
    #[test_case(2400)]
    fn years_divisible_by_four_hundred_are_leap_years(year: u16) {
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
    fn years_divisible_by_four_but_not_by_one_hundred_are_leap_years(year: u16) {
        assert_eq!(true, is_leap(year));
    }
}
