use std::fmt;
use std::str::FromStr;

enum Direction {
    North,
    West,
    South,
    East,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "N" => Ok(Direction::North),
            "W" => Ok(Direction::West),
            "S" => Ok(Direction::South),
            "E" => Ok(Direction::East),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::North => write!(f, "N"),
            Direction::South => write!(f, "S"),
            Direction::West => write!(f, "W"),
            Direction::East => write!(f, "E"),
        }
    }
}

enum Command {
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

struct Coordinate {
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

struct Position {
    coordinate: Coordinate,
    direction: Direction,
}

impl Position {
    fn new(coordinate: Coordinate, direction: Direction) -> Self {
        Self {
            coordinate,
            direction,
        }
    }

    fn turn_left(&mut self) {
        match self.direction {
            Direction::North => self.change_direction(Direction::West),
            Direction::West => self.change_direction(Direction::South),
            Direction::South => self.change_direction(Direction::East),
            Direction::East => self.change_direction(Direction::North),
        }
    }

    fn turn_right(&mut self) {
        match self.direction {
            Direction::North => self.change_direction(Direction::East),
            Direction::West => self.change_direction(Direction::North),
            Direction::South => self.change_direction(Direction::West),
            Direction::East => self.change_direction(Direction::South),
        }
    }

    fn move_position(&mut self) {
        match self.direction {
            Direction::North => self.move_north(),
            Direction::West => self.move_west(),
            Direction::South => self.move_south(),
            Direction::East => self.move_east(),
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn move_north(&mut self) {
        self.coordinate.increment_y()
    }

    fn move_south(&mut self) {
        self.coordinate.decrement_y()
    }

    fn move_east(&mut self) {
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

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    fn parse(&self, instructions: String) -> (Position, Vec<Command>) {
        let lines: Vec<&str> = instructions.lines().collect();
        let position = self.parse_position(lines[1]);

        if lines.len() < 3 {
            return (position, Vec::new());
        }

        let commands = self.parse_commands(lines[2]);

        (position, commands)
    }

    fn parse_position(&self, position: &str) -> Position {
        let position_parts: Vec<&str> = position.split_whitespace().collect();
        let x: u8 = position_parts[0].parse().unwrap_or(0);
        let y: u8 = position_parts[1].parse().unwrap_or(0);
        let direction = position_parts[2];

        Position::new(
            Coordinate::new(x, y),
            Direction::from_str(direction).unwrap_or(Direction::North),
        )
    }

    fn parse_commands(&self, commands: &str) -> Vec<Command> {
        let commands: Vec<char> = commands.chars().collect();
        return commands.iter().map(Command::from).collect();
    }
}

pub struct Rover {
    position: Position,
    parser: Parser,
}

impl Rover {
    pub fn new(parser: Parser) -> Self {
        Rover {
            position: Position::new(Coordinate::new(0, 0), Direction::North),
            parser,
        }
    }

    pub fn execute(&mut self, instructions: String) -> String {
        let (starting_position, commands) = self.parser.parse(instructions);
        self.update_position(starting_position);

        for c in commands {
            match c {
                Command::Left => self.turn_left(),
                Command::Right => self.turn_right(),
                Command::Move => self.move_rover(),
                _ => {}
            }
        }

        format!("{}", self.position)
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
}

#[cfg(test)]
mod mars_rover_unit_tests {
    use crate::mars_rover::*;
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
    use crate::mars_rover::*;
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
