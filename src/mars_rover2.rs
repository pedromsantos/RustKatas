use std::fmt;

struct DirectionMoveVector(i8, i8);

trait Direction {
    fn turn_left(&self) -> Box<dyn Direction>;
    fn turn_right(&self) -> Box<dyn Direction>;
    fn move_vector(&self) -> DirectionMoveVector;
    fn to_string(&self) -> String;
}

struct North;
struct South;
struct West;
struct East;

impl Direction for North {
    fn turn_left(&self) -> Box<dyn Direction> {
        Box::new(West)
    }
    fn turn_right(&self) -> Box<dyn Direction> {
        Box::new(East)
    }

    fn move_vector(&self) -> DirectionMoveVector {
        return DirectionMoveVector(0, 1);
    }

    fn to_string(&self) -> String {
        return String::from("N");
    }
}

impl Direction for South {
    fn turn_left(&self) -> Box<dyn Direction> {
        Box::new(East)
    }
    fn turn_right(&self) -> Box<dyn Direction> {
        Box::new(West)
    }

    fn move_vector(&self) -> DirectionMoveVector {
        return DirectionMoveVector(0, -1);
    }

    fn to_string(&self) -> String {
        return String::from("S");
    }
}

impl Direction for East {
    fn turn_left(&self) -> Box<dyn Direction> {
        Box::new(North)
    }

    fn turn_right(&self) -> Box<dyn Direction> {
        Box::new(South)
    }

    fn move_vector(&self) -> DirectionMoveVector {
        return DirectionMoveVector(1, 0);
    }

    fn to_string(&self) -> String {
        return String::from("E");
    }
}

impl Direction for West {
    fn turn_left(&self) -> Box<dyn Direction> {
        Box::new(South)
    }
    fn turn_right(&self) -> Box<dyn Direction> {
        Box::new(North)
    }

    fn move_vector(&self) -> DirectionMoveVector {
        return DirectionMoveVector(-1, 0);
    }

    fn to_string(&self) -> String {
        return String::from("W");
    }
}

struct DirectionFactory;

impl DirectionFactory {
    fn create(input: &str) -> Box<dyn Direction> {
        match input {
            "N" => Box::new(North),
            "W" => Box::new(West),
            "S" => Box::new(South),
            "E" => Box::new(East),
            _ => Box::new(North),
        }
    }
}

trait RoverCommand {
    fn execute(&self, rover: &mut Rover);
}

struct MoveForward;

struct TurnLeft;

struct TurnRight;

struct DoNothing;

impl RoverCommand for MoveForward {
    fn execute(&self, rover: &mut Rover) {
        rover.move_forward();
    }
}

impl RoverCommand for TurnLeft {
    fn execute(&self, rover: &mut Rover) {
        rover.turn_left();
    }
}

impl RoverCommand for TurnRight {
    fn execute(&self, rover: &mut Rover) {
        rover.turn_right();
    }
}

impl RoverCommand for DoNothing {
    fn execute(&self, _: &mut Rover) {}
}

struct Commands {
    commands: Vec<Box<dyn RoverCommand>>,
}

impl Commands {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add(&mut self, cmd: Box<dyn RoverCommand>) {
        self.commands.push(cmd);
    }

    fn execute(&self, rover: &mut Rover) {
        self.commands.iter().for_each(|c| c.execute(rover));
    }

    fn create_command(input: &char) -> Box<dyn RoverCommand> {
        match input {
            'M' => Box::new(MoveForward),
            'L' => Box::new(TurnLeft),
            'R' => Box::new(TurnRight),
            _ => Box::new(DoNothing),
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

    pub fn apply_vector(&mut self, vector: DirectionMoveVector) {
        self.x = (self.x as i8 + vector.0) as u8;
        self.y = (self.y as i8 + vector.1) as u8;
    }
}

struct Position {
    coordinate: Coordinate,
    direction: Box<dyn Direction>,
}

impl Position {
    pub fn new(coordinate: Coordinate, direction: Box<dyn Direction>) -> Self {
        Self {
            coordinate,
            direction,
        }
    }

    pub fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    pub fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    pub fn move_forward(&mut self) {
        self.coordinate.apply_vector(self.direction.move_vector())
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.coordinate, self.direction.to_string())
    }
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    fn parse(&self, instructions: String) -> (Commands, Position) {
        let lines: Vec<&str> = instructions.lines().collect();
        let position = self.parse_position(lines[1]);

        if lines.len() < 3 {
            return (Commands::new(), position);
        }

        let commands = self.parse_commands(lines[2]);

        return (commands, position);
    }

    fn parse_position(&self, position: &str) -> Position {
        let position_parts: Vec<&str> = position.split_whitespace().collect();
        let x: u8 = position_parts[0].parse().unwrap_or(0);
        let y: u8 = position_parts[1].parse().unwrap_or(0);
        let direction = position_parts[2];

        return Position::new(Coordinate::new(x, y), DirectionFactory::create(direction));
    }

    fn parse_commands(&self, commands: &str) -> Commands {
        let raw_commands: Vec<char> = commands.chars().collect();
        let mut commands = Commands::new();

        raw_commands
            .iter()
            .for_each(|c| commands.add(Commands::create_command(c)));

        return commands;
    }
}

pub struct Rover {
    parser: Parser,
    position: Position,
}

impl Rover {
    pub fn new(parser: Parser) -> Self {
        Rover {
            position: Position::new(Coordinate::new(0, 0), Box::new(North)),
            parser,
        }
    }

    pub fn execute(&mut self, instructions: String) -> String {
        let (commands, starting_position) = self.parser.parse(instructions);
        self.update_position(starting_position);

        commands.execute(self);

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

    fn move_forward(&mut self) {
        self.position.move_forward();
    }
}

#[cfg(test)]
mod mars_rover_2_unit_tests {
    use crate::mars_rover2::*;
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
mod mars_rover_2_acceptance_tests {
    use crate::mars_rover2::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn turn_left_and_move() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n1 2 N\nLMLMLMLMM"));

        assert_eq!(String::from("1 3 N"), position);
    }

    #[test]
    fn turn_right_and_move() {
        let mut rover = Rover::new(Parser::new());

        let position = rover.execute(String::from("5 5\n3 3 E\nMMRMMRMRRM"));

        assert_eq!(String::from("5 1 E"), position);
    }
}
