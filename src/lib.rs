pub mod fizz_buzz {
    pub fn fizz_buzzer(number: u8) -> String {
        match (number % 3, number % 5) {
            (0, 0) => String::from("fizzbuzz"),
            (0, _) => String::from("fizz"),
            (_, 0) => String::from("buzz"),
            (_, _) => number.to_string(),
        }
    }
}

#[cfg(test)]
mod fizz_buzzer_tests {
    use super::fizz_buzz::*;
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

    prop_compose! {
        fn multiples_of_fifteen(max: u8)(base in 0..max/5) -> u8 { base * 15 }
    }

    prop_compose! {
        fn multiples_of_five(max: u8)(base in 0..max) -> u8 { base * 5 }
    }

    prop_compose! {
        fn multiples_of_three(max: u8)(base in 0..max) -> u8 { base * 3 }
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

#[cfg(test)]
mod leap_year_tests {
    use super::leap_year::*;
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

pub mod fibonacci_sequence {
    pub fn fibonacci(index: u16) -> u16 {
        match index {
            0 => 0,
            1 => 1,
            i => fibonacci(i - 1) + fibonacci(i - 2),
        }
    }
}

#[cfg(test)]
mod fibonacci_tests {
    use super::fibonacci_sequence::*;
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

pub mod roman_numerals {
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
}

#[cfg(test)]
mod roman_numerals_tests {
    use super::roman_numerals::*;
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

pub mod tic_tac_toe {
    use std::collections::HashMap;

    #[derive(PartialEq, Debug)]
    pub enum Status {
        Playing,
        Win(Player),
        Draw,
    }

    #[derive(PartialEq, Clone, Copy, Debug)]
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
        Bottom,
    }

    #[derive(PartialEq, Eq, Hash, Clone, Copy)]
    pub struct Square {
        row: Row,
        column: Column,
    }

    impl Square {
        fn new(row: Row, column: Column) -> Self {
            return Square { row, column };
        }

        pub fn top_left() -> Square {
            return Square::new(Row::Top, Column::Left);
        }

        pub fn top_middle() -> Square {
            return Square::new(Row::Top, Column::Middle);
        }

        pub fn top_rigth() -> Square {
            return Square::new(Row::Top, Column::Rigth);
        }

        pub fn center_left() -> Square {
            return Square::new(Row::Center, Column::Left);
        }

        pub fn center_middle() -> Square {
            return Square::new(Row::Center, Column::Middle);
        }

        pub fn center_rigth() -> Square {
            return Square::new(Row::Center, Column::Rigth);
        }

        pub fn bottom_left() -> Square {
            return Square::new(Row::Bottom, Column::Left);
        }

        pub fn bottom_middle() -> Square {
            return Square::new(Row::Bottom, Column::Middle);
        }

        pub fn bottom_rigth() -> Square {
            return Square::new(Row::Bottom, Column::Rigth);
        }
    }

    struct Board {
        movements: HashMap<Square, Player>,
    }

    impl Default for Board {
        fn default() -> Self {
            Board {
                movements: HashMap::with_capacity(9),
            }
        }
    }

    impl Board {
        pub fn add(&mut self, player: Player, square: Square) -> Result<(), String> {
            if self.movements.contains_key(&square) {
                return Err(String::from("Invalid move"));
            }

            self.movements.insert(square, player);

            Ok(())
        }

        pub fn is_same_player_in_all_squares_in_row_or_column(
            &self,
            player: Player,
            square: Square,
        ) -> bool {
            return self.is_same_player_in_all_squares_in_row(&player, square.row)
                || self.is_same_player_in_all_squares_in_column(&player, square.column);
        }

        fn is_same_player_in_all_squares_in_row(&self, player: &Player, row: Row) -> bool {
            self.movements
                .iter()
                .filter(|m| m.1 == player && m.0.row == row)
                .count()
                == 3
        }

        fn is_same_player_in_all_squares_in_column(&self, player: &Player, column: Column) -> bool {
            self.movements
                .iter()
                .filter(|m| m.1 == player && m.0.column == column)
                .count()
                == 3
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
        pub fn play(&mut self, player: Player, square: Square) -> Result<Status, String> {
            if player == self.last_player {
                return Err(String::from("Invalid player"));
            }

            self.board.add(player, square)?;

            if self
                .board
                .is_same_player_in_all_squares_in_row_or_column(player, square)
            {
                return Ok(Status::Win(player));
            }

            self.last_player = player;

            Ok(Status::Playing)
        }
    }
}

#[cfg(test)]
mod tic_tac_toe_should {
    use super::tic_tac_toe::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn not_allow_player_o_to_play_first() {
        let mut game = Game::default();

        let result = game.play(Player::O, Square::top_left());

        assert_eq!(Err(String::from("Invalid player")), result);
    }

    #[test]
    fn not_allow_player_x_to_play_twice() {
        let mut game = Game::default();

        let mut result = game.play(Player::X, Square::top_left());
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, Square::top_middle());
        assert_eq!(Err(String::from("Invalid player")), result);
    }

    #[test]
    fn not_allow_player_to_play_twice_in_same_position() {
        let mut game = Game::default();

        let mut result = game.play(Player::X, Square::top_left());
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, Square::top_left());
        assert_eq!(Err(String::from("Invalid move")), result);
    }

    #[test]
    fn not_allow_player_to_play_in_same_position_once_taken() {
        let mut game = Game::default();

        let mut result = game.play(Player::X, Square::top_left());
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::O, Square::top_middle());
        assert_eq!(Ok(Status::Playing), result);

        result = game.play(Player::X, Square::top_left());
        assert_eq!(Err(String::from("Invalid move")), result);
    }

    #[test]
    fn declare_winner_if_it_has_three_moves_on_top_row() {
        let mut game = Game::default();

        _ = game.play(Player::X, Square::top_left());
        _ = game.play(Player::O, Square::center_left());
        _ = game.play(Player::X, Square::top_middle());
        _ = game.play(Player::O, Square::center_middle());
        let result = game.play(Player::X, Square::top_rigth());
        assert_eq!(Ok(Status::Win(Player::X)), result);
    }

    #[test]
    fn declare_winner_if_it_has_three_moves_on_center_row() {
        let mut game = Game::default();

        _ = game.play(Player::X, Square::center_left());
        _ = game.play(Player::O, Square::top_left());
        _ = game.play(Player::X, Square::center_middle());
        _ = game.play(Player::O, Square::top_middle());
        let result = game.play(Player::X, Square::center_rigth());
        assert_eq!(Ok(Status::Win(Player::X)), result);
    }

    #[test]
    fn declare_winner_if_it_has_three_moves_on_bottom_row() {
        let mut game = Game::default();

        _ = game.play(Player::X, Square::bottom_left());
        _ = game.play(Player::O, Square::top_left());
        _ = game.play(Player::X, Square::bottom_middle());
        _ = game.play(Player::O, Square::top_middle());
        let result = game.play(Player::X, Square::bottom_rigth());
        assert_eq!(Ok(Status::Win(Player::X)), result);
    }

    #[test]
    fn declare_winner_if_it_has_three_moves_on_left_column() {
        let mut game = Game::default();

        _ = game.play(Player::X, Square::top_left());
        _ = game.play(Player::O, Square::center_rigth());
        _ = game.play(Player::X, Square::center_left());
        _ = game.play(Player::O, Square::center_middle());
        let result = game.play(Player::X, Square::bottom_left());
        assert_eq!(Ok(Status::Win(Player::X)), result);
    }

    #[test]
    fn declare_winner_if_it_has_three_moves_on_middle_column() {
        let mut game = Game::default();

        _ = game.play(Player::X, Square::top_middle());
        _ = game.play(Player::O, Square::center_rigth());
        _ = game.play(Player::X, Square::center_middle());
        _ = game.play(Player::O, Square::top_rigth());
        let result = game.play(Player::X, Square::bottom_middle());
        assert_eq!(Ok(Status::Win(Player::X)), result);
    }

    #[test]
    fn declare_winner_if_it_has_three_moves_on_right_column() {
        let mut game = Game::default();

        _ = game.play(Player::X, Square::top_rigth());
        _ = game.play(Player::O, Square::center_left());
        _ = game.play(Player::X, Square::center_rigth());
        _ = game.play(Player::O, Square::center_middle());
        let result = game.play(Player::X, Square::bottom_rigth());
        assert_eq!(Ok(Status::Win(Player::X)), result);
    }
}

pub mod mars_rover {
    use std::fmt;
    use std::str::FromStr;

    pub enum Direction {
        NORTH,
        WEST,
        SOUTH,
        EAST,
    }

    impl FromStr for Direction {
        type Err = ();

        fn from_str(input: &str) -> Result<Direction, Self::Err> {
            match input {
                "N" => Ok(Direction::NORTH),
                "W" => Ok(Direction::WEST),
                "S" => Ok(Direction::SOUTH),
                "E" => Ok(Direction::EAST),
                _ => Err(()),
            }
        }
    }

    impl fmt::Display for Direction {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Direction::NORTH => write!(f, "N"),
                Direction::SOUTH => write!(f, "S"),
                Direction::WEST => write!(f, "W"),
                Direction::EAST => write!(f, "E"),
            }
        }
    }

    pub enum Command {
        Left,
        Right,
        Move,
        Null,
    }

    impl Command {
        fn from(input: &char) -> Command {
            match input {
                'L' => Command::Left,
                'R' => Command::Right,
                'M' => Command::Move,
                _ => Command::Null,
            }
        }
    }

    pub struct Coordinate {
        x: u8,
        y: u8,
    }

    impl fmt::Display for Coordinate {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {}", self.x, self.y)
        }
    }

    impl Coordinate {
        pub fn new(x: u8, y: u8) -> Self {
            Self { x, y }
        }

        pub fn increment_y(&mut self) {
            self.y += 1;
        }

        pub fn decrement_y(&mut self) {
            self.y -= 1;
        }

        pub fn increment_x(&mut self) {
            self.x += 1;
        }

        pub fn decrement_x(&mut self) {
            self.x -= 1;
        }
    }

    pub struct Position {
        coordinate: Coordinate,
        direction: Direction,
    }

    impl Position {
        pub fn new(coordinate: Coordinate, direction: Direction) -> Self {
            Self {
                coordinate: coordinate,
                direction: direction,
            }
        }

        pub fn turn_left(&mut self) {
            match self.direction {
                Direction::NORTH => self.change_direction(Direction::WEST),
                Direction::WEST => self.change_direction(Direction::SOUTH),
                Direction::SOUTH => self.change_direction(Direction::EAST),
                Direction::EAST => self.change_direction(Direction::NORTH),
            }
        }

        pub fn turn_right(&mut self) {
            match self.direction {
                Direction::NORTH => self.change_direction(Direction::EAST),
                Direction::WEST => self.change_direction(Direction::NORTH),
                Direction::SOUTH => self.change_direction(Direction::WEST),
                Direction::EAST => self.change_direction(Direction::SOUTH),
            }
        }

        pub fn move_position(&mut self) {
            match self.direction {
                Direction::NORTH => self.move_north(),
                Direction::WEST => self.move_west(),
                Direction::SOUTH => self.move_south(),
                Direction::EAST => self.move_east(),
            }
        }

        fn change_direction(&mut self, direction: Direction) {
            self.direction = direction;
        }

        pub fn move_north(&mut self) {
            self.coordinate.increment_y()
        }

        pub fn move_south(&mut self) {
            self.coordinate.decrement_y()
        }

        pub fn move_east(&mut self) {
            self.coordinate.increment_x()
        }

        pub fn move_west(&mut self) {
            self.coordinate.decrement_x()
        }
    }

    impl fmt::Display for Position {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {}", self.coordinate, self.direction)
        }
    }

    pub struct Parser {}

    impl Parser {
        pub fn new() -> Self {
            Parser {}
        }

        pub fn parse(&self, instructions: String) -> (Position, Vec<Command>) {
            let lines: Vec<&str> = instructions.lines().collect();
            let position = self.parse_position(lines[1]);

            if lines.len() < 3 {
                return (position, Vec::new());
            }

            let commands = self.parse_commands(lines[2]);

            return (position, commands);
        }

        fn parse_position(&self, position: &str) -> Position {
            let position_parts: Vec<&str> = position.split_whitespace().collect();
            let x: u8 = position_parts[0].parse().unwrap_or(0);
            let y: u8 = position_parts[1].parse().unwrap_or(0);
            let direction = position_parts[2];

            return Position::new(
                Coordinate::new(x, y),
                Direction::from_str(direction).unwrap_or(Direction::NORTH),
            );
        }

        fn parse_commands(&self, commands: &str) -> Vec<Command> {
            let commands: Vec<char> = commands.chars().collect();
            return commands.iter().map(|c| Command::from(c)).collect();
        }
    }

    pub struct Rover {
        position: Position,
        parser: Parser,
    }

    impl Rover {
        pub fn new(parser: Parser) -> Self {
            Rover {
                position: Position::new(Coordinate::new(0, 0), Direction::NORTH),
                parser: parser,
            }
        }

        pub fn execute(&mut self, instructions: String) -> String {
            let (position, commands) = self.parser.parse(instructions);
            self.update_position(position);

            for c in commands {
                match c {
                    Command::Left => self.turn_left(),
                    Command::Right => self.turn_right(),
                    Command::Move => self.move_rover(),
                    _ => {}
                }
            }

            return self.print_final_position();
        }

        fn update_position(&mut self, position: Position) {
            self.position = position;
        }

        fn turn_left(&mut self) {
            self.position.turn_left();
        }

        fn turn_right(&mut self) {
            self.position.turn_right();
        }

        fn move_rover(&mut self) {
            self.position.move_position();
        }

        fn print_final_position(&self) -> String {
            self.position.to_string()
        }
    }
}

#[cfg(test)]
mod mars_rover_unit_tests {
    use super::mars_rover::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn stays_in_same_position_when_no_commands_are_sent() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 N\n"));

        assert_eq!(String::from("1 1 N"), position);
    }

    #[test]
    fn turn_left_turns_from_north_to_west() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 N\nL"));

        assert_eq!(String::from("1 1 W"), position);
    }

    #[test]
    fn turn_left_turns_from_west_to_south() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 W\nL"));

        assert_eq!(String::from("1 1 S"), position);
    }

    #[test]
    fn turn_left_turns_from_south_to_east() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 S\nL"));

        assert_eq!(String::from("1 1 E"), position);
    }

    #[test]
    fn turn_left_turns_from_east_to_north() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 E\nL"));

        assert_eq!(String::from("1 1 N"), position);
    }

    #[test]
    fn turn_left_four_time_ends_in_same_position() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 N\nLLLL"));

        assert_eq!(String::from("1 1 N"), position);
    }

    #[test]
    fn turn_right_turns_from_north_to_east() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 N\nR"));

        assert_eq!(String::from("1 1 E"), position);
    }

    #[test]
    fn turn_right_turns_from_east_to_south() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 E\nR"));

        assert_eq!(String::from("1 1 S"), position);
    }

    #[test]
    fn turn_right_turns_from_south_to_west() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 S\nR"));

        assert_eq!(String::from("1 1 W"), position);
    }

    #[test]
    fn turn_right_turns_from_west_to_north() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 W\nR"));

        assert_eq!(String::from("1 1 N"), position);
    }

    #[test]
    fn turn_right_four_time_ends_in_same_position() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 N\nRRRR"));

        assert_eq!(String::from("1 1 N"), position);
    }

    #[test]
    fn move_north() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 N\nM"));

        assert_eq!(String::from("1 2 N"), position);
    }

    #[test]
    fn move_south() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 S\nM"));

        assert_eq!(String::from("1 0 S"), position);
    }

    #[test]
    fn move_west() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 W\nM"));

        assert_eq!(String::from("0 1 W"), position);
    }

    #[test]
    fn move_east() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 1 E\nM"));

        assert_eq!(String::from("2 1 E"), position);
    }
}

#[cfg(test)]
mod mars_rover_acceptance_tests {
    use super::mars_rover::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn turn_left_and_move() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 2 N\nLMLMLMLMM"));

        assert_eq!(String::from("1 3 N"), position);
    }

    #[test]
    fn turn_rigth_and_move() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n3 3 E\nMMRMMRMRRM"));

        assert_eq!(String::from("5 1 E"), position);
    }
}
