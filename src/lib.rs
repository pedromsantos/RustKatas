pub mod fizz_buzz {
    use std::borrow::Cow;

    pub fn fizz_buzzer(number: u8) -> Cow<'static, str> {
        match (number % 3, number % 5) {
            (0, 0) => "fizzbuzz".into(),
            (0, _) => "fizz".into(),
            (_, 0) => "buzz".into(),
            (_, _) => number.to_string().into(),
        }
    }
}

pub mod leap_year {
    pub fn is_leap(year: u16) -> bool {
        match (year % 400, year % 4, year % 100) {
            (0, _, _) => true,
            (_, 0, r) => r != 0,
            _ => false,
        }
    }
}

pub mod fibonacci_sequence {
    pub fn fibonacci(index: u16) -> u16 {
        match index {
            0 => 0,
            1 => 1,
            i => fibonacci(i - 1) + fibonacci(i - 2),
        }
    }
}

pub mod roman_numerals {
    const ARABIC_NUMBERS_TO_ROMAN_NUMERALS: [(u16, &str); 13] = [
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
        for (arabic_number, roman_nuneral) in ARABIC_NUMBERS_TO_ROMAN_NUMERALS.iter() {
            if number >= *arabic_number {
                return (*roman_nuneral).to_string() + &to_roman_numeral(number - arabic_number);
            }
        }

        "".to_string()
    }
}

pub mod tic_tac_toe {
    use std::collections::HashMap;
    use std::fmt;

    #[derive(Debug, PartialEq)]
    pub struct InvalidMove;

    impl fmt::Display for InvalidMove {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Invalid move")
        }
    }

    fn invalid_move() -> Result<Status, InvalidMove> {
        Err(InvalidMove)
    }

    #[derive(PartialEq, Debug)]
    pub enum Status {
        Playing,
        Win,
        Draw,
    }

    #[derive(PartialEq, Clone, Copy)]
    pub enum Player {
        X,
        O,
    }

    #[derive(PartialEq, Eq, Hash, Clone, Copy)]
    pub enum Column {
        Left,
        Middle,
        Rigth,
    }

    #[derive(PartialEq, Eq, Hash, Clone, Copy)]
    pub enum Row {
        Top,
        Center,
        Bottom
    }

    #[derive(PartialEq, Eq, Hash, Clone, Copy)]
    struct Move {
        row: Row,
        column: Column,
    }

    struct Board {
        last_movements: HashMap<Move, Player>,
    }

    impl Default for Board {
        fn default() -> Self {
            Board {
                last_movements: HashMap::new(),
            }
        }
    }

    impl Board {
        pub fn add(&mut self, player: Player, movement: (Row, Column)) -> Result<Status, InvalidMove> {
            let moving = Move {
                row: movement.0,
                column: movement.1,
            };

            if self.last_movements.contains_key(&moving) {
                return invalid_move();
            }

            self.last_movements.insert(moving, player);

            if self.player_wins(movement, player) {
                return Ok(Status::Win);
            }

            return Ok(Status::Playing);
        }

        fn player_wins(&self, movement: (Row, Column), player: Player) -> bool {
            if self.player_wins_in_row(movement.0, player) || 
                self.player_wins_in_column(movement.1, player) {
                return true;
            }

            return false;
        }

        fn player_wins_in_row(&self, row: Row, player: Player) -> bool {
            return
                self.last_movements
                .iter()
                .filter(|&m| *m.1 == player && m.0.row == row)
                .count() == 3;
        }

        fn player_wins_in_column(&self, column: Column, player: Player) -> bool {
            return
                self.last_movements
                .iter()
                .filter(|&m| *m.1 == player && m.0.column == column)
                .count() == 3;
        }
    }

    pub struct Game {
        last_player: Player,
        board: Board,
    }

    impl Default for Game {
        fn default() -> Self {
            Game {
                last_player: Player::O,
                board: Board::default(),
            }
        }
    }

    impl Game {
        pub fn play(
            &mut self,
            player: Player,
            movement: (Row, Column),
        ) -> Result<Status, InvalidMove> {
            if player == self.last_player {
                return invalid_move();
            }

            self.last_player = player.clone();

            return self.board.add(player, movement);
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
    fn convert_non_multiples_of_three_or_and_five_to_text(number: u8, expected: &'static str) {
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
}

#[cfg(test)]
mod leap_year_tests {
    use super::leap_year::*;
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
    fn fibonacci_number_is_the_sum_of_the_two_preceding_ones(index: u16, fibonacci_number: u16) {
        assert_eq!(fibonacci_number, fibonacci(index));
    }
}

#[cfg(test)]
mod roman_numerals_tests {
    use super::roman_numerals::*;
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

#[cfg(test)]
mod tic_tac_toe_tests {
    use super::tic_tac_toe::*;

    #[test]
    fn should_not_alow_player_o_to_play_first() {
        let mut game = Game::default();

        let result = game.play(Player::O, (Row::Top, Column::Left));

        assert_eq!(Err(InvalidMove), result);
    }

    #[test]
    fn should_not_alow_player_x_to_play_twice() {
        let mut game = Game::default();

        let mut result = game.play(Player::X, (Row::Top, Column::Left));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, (Row::Top, Column::Middle));
        assert_eq!(Err(InvalidMove), result);
    }

    #[test]
    fn should_not_alow_player_to_play_twice_in_same_position() {
        let mut game = Game::default();

        let mut result = game.play(Player::X, (Row::Top, Column::Left));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, (Row::Top, Column::Left));
        assert_eq!(Err(InvalidMove), result);
    }

    #[test]
    fn should_not_alow_player_to_play_in_same_position_once_taken() {
        let mut game = Game::default();

        let mut result = game.play(Player::X, (Row::Top, Column::Left));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, (Row::Top, Column::Middle));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, (Row::Top, Column::Left));
        assert_eq!(Err(InvalidMove), result);
    }

    #[test]
    fn should_declare_winner_if_it_has_three_moves_on_top_row() {
        let mut game = Game::default();

        let mut result = game.play(Player::X, (Row::Top, Column::Left));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, (Row::Center, Column::Left));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, (Row::Top, Column::Middle));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, (Row::Center, Column::Middle));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, (Row::Top, Column::Rigth));
        assert_eq!(Ok(Status::Win), result);
    }

    #[test]
    fn should_declare_winner_if_it_has_three_moves_on_center_row() {
        let mut game = Game::default();

        let mut result = game.play(Player::X, (Row::Center, Column::Left));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, (Row::Top, Column::Left));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, (Row::Center, Column::Middle));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, (Row::Top, Column::Middle));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, (Row::Center, Column::Rigth));
        assert_eq!(Ok(Status::Win), result);
    }

    #[test]
    fn should_declare_winner_if_it_has_three_moves_on_bottom_row() {
        let mut game = Game::default();

        let mut result = game.play(Player::X, (Row::Bottom, Column::Left));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, (Row::Top, Column::Left));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, (Row::Bottom, Column::Middle));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, (Row::Top, Column::Middle));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, (Row::Bottom, Column::Rigth));
        assert_eq!(Ok(Status::Win), result);
    }

    #[test]
    fn should_declare_winner_if_it_has_three_moves_on_left_column() {
        let mut game = Game::default();

        let mut result = game.play(Player::X, (Row::Top, Column::Left));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, (Row::Center, Column::Rigth));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, (Row::Center, Column::Left));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, (Row::Center, Column::Middle));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, (Row::Bottom, Column::Left));
        assert_eq!(Ok(Status::Win), result);
    }

    #[test]
    fn should_declare_winner_if_it_has_three_moves_on_middle_column() {
        let mut game = Game::default();

        let mut result = game.play(Player::X, (Row::Top, Column::Middle));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, (Row::Center, Column::Rigth));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, (Row::Center, Column::Middle));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, (Row::Top, Column::Rigth));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, (Row::Bottom, Column::Middle));
        assert_eq!(Ok(Status::Win), result);
    }

    #[test]
    fn should_declare_winner_if_it_has_three_moves_on_right_column() {
        let mut game = Game::default();

        let mut result = game.play(Player::X, (Row::Top, Column::Rigth));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, (Row::Center, Column::Left));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, (Row::Center, Column::Rigth));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, (Row::Center, Column::Middle));
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, (Row::Bottom, Column::Rigth));
        assert_eq!(Ok(Status::Win), result);
    }
}
