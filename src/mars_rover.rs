use std::fmt;
use std::str::FromStr;

enum Direction {
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

pub struct Position {
    coordinate: Coordinate,
    direction: Direction,
}

impl Position {
    fn new(coordinate: Coordinate, direction: Direction) -> Self {
        Self {
            coordinate: coordinate,
            direction: direction,
        }
    }

    fn turn_left(&mut self) {
        match self.direction {
            Direction::NORTH => self.change_direction(Direction::WEST),
            Direction::WEST => self.change_direction(Direction::SOUTH),
            Direction::SOUTH => self.change_direction(Direction::EAST),
            Direction::EAST => self.change_direction(Direction::NORTH),
        }
    }

    fn turn_right(&mut self) {
        match self.direction {
            Direction::NORTH => self.change_direction(Direction::EAST),
            Direction::WEST => self.change_direction(Direction::NORTH),
            Direction::SOUTH => self.change_direction(Direction::WEST),
            Direction::EAST => self.change_direction(Direction::SOUTH),
        }
    }

    fn move_position(&mut self) {
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

        return format!("{}", self.position);
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
