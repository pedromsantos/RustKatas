use std::collections::HashMap;

#[derive(PartialEq)]
pub enum Status {
    Draw,
    Playing,
    Win(Player),
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
    Bottom,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Square {
    row: Row,
    column: Column,
}

impl Square {
    fn new(row: Row, column: Column) -> Self {
        Square { row, column }
    }

    pub fn top_left() -> Square {
        Square::new(Row::Top, Column::Left)
    }

    pub fn top_middle() -> Square {
        Square::new(Row::Top, Column::Middle)
    }

    pub fn top_rigth() -> Square {
        Square::new(Row::Top, Column::Rigth)
    }

    pub fn center_left() -> Square {
        Square::new(Row::Center, Column::Left)
    }

    pub fn center_middle() -> Square {
        Square::new(Row::Center, Column::Middle)
    }

    pub fn center_rigth() -> Square {
        Square::new(Row::Center, Column::Rigth)
    }

    pub fn bottom_left() -> Square {
        Square::new(Row::Bottom, Column::Left)
    }

    pub fn bottom_middle() -> Square {
        Square::new(Row::Bottom, Column::Middle)
    }

    pub fn bottom_rigth() -> Square {
        Square::new(Row::Bottom, Column::Rigth)
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

    pub fn is_same_player_in_square_row_or_column(&self, player: Player, square: Square) -> bool {
        self.is_same_player_in_all_squares_in_row(&player, square.row)
            || self.is_same_player_in_all_squares_in_column(&player, square.column)
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
            .is_same_player_in_square_row_or_column(player, square)
        {
            return Ok(Status::Win(player));
        }

        self.last_player = player;

        Ok(Status::Playing)
    }
}

#[cfg(test)]
mod tic_tac_toe_should {
    use super::*;

    #[test]
    fn not_allow_player_o_to_play_first() {
        let mut game = Game::default();

        assert!(matches!(
            game.play(Player::O, Square::top_left()),
            Err(ref e) if e == "Invalid player"));
    }

    #[test]
    fn not_allow_player_x_to_play_twice() {
        let mut game = Game::default();

        assert!(matches!(
            game.play(Player::X, Square::top_left()),
            Ok(Status::Playing)
        ));

        assert!(matches!(
            game.play(Player::X, Square::top_middle()),
            Err(ref e) if e == "Invalid player"));
    }

    #[test]
    fn not_allow_player_to_play_twice_in_same_position() {
        let mut game = Game::default();

        assert!(matches!(
            game.play(Player::X, Square::top_left()),
            Ok(Status::Playing)
        ));

        assert!(matches!(
            game.play(Player::O, Square::top_left()),
            Err(ref e) if e == "Invalid move"));
    }

    #[test]
    fn not_allow_player_to_play_in_same_position_once_taken() {
        let mut game = Game::default();

        assert!(matches!(
            game.play(Player::X, Square::top_left()),
            Ok(Status::Playing)
        ));

        assert!(matches!(
            game.play(Player::O, Square::top_middle()),
            Ok(Status::Playing)
        ));

        assert!(matches!(
            game.play(Player::X, Square::top_left()),
            Err(ref e) if e == "Invalid move"));
    }

    #[test]
    fn declare_winner_if_it_has_three_moves_on_top_row() {
        let mut game = Game::default();

        _ = game.play(Player::X, Square::top_left());
        _ = game.play(Player::O, Square::center_left());
        _ = game.play(Player::X, Square::top_middle());
        _ = game.play(Player::O, Square::center_middle());

        assert!(matches!(
            game.play(Player::X, Square::top_rigth()),
            Ok(Status::Win(Player::X))
        ));
    }

    #[test]
    fn declare_winner_if_it_has_three_moves_on_center_row() {
        let mut game = Game::default();

        _ = game.play(Player::X, Square::center_left());
        _ = game.play(Player::O, Square::top_left());
        _ = game.play(Player::X, Square::center_middle());
        _ = game.play(Player::O, Square::top_middle());

        assert!(matches!(
            game.play(Player::X, Square::center_rigth()),
            Ok(Status::Win(Player::X))
        ));
    }

    #[test]
    fn declare_winner_if_it_has_three_moves_on_bottom_row() {
        let mut game = Game::default();

        _ = game.play(Player::X, Square::bottom_left());
        _ = game.play(Player::O, Square::top_left());
        _ = game.play(Player::X, Square::bottom_middle());
        _ = game.play(Player::O, Square::top_middle());

        assert!(matches!(
            game.play(Player::X, Square::bottom_rigth()),
            Ok(Status::Win(Player::X))
        ));
    }

    #[test]
    fn declare_winner_if_it_has_three_moves_on_left_column() {
        let mut game = Game::default();

        _ = game.play(Player::X, Square::top_left());
        _ = game.play(Player::O, Square::center_rigth());
        _ = game.play(Player::X, Square::center_left());
        _ = game.play(Player::O, Square::center_middle());

        assert!(matches!(
            game.play(Player::X, Square::bottom_left()),
            Ok(Status::Win(Player::X))
        ));
    }

    #[test]
    fn declare_winner_if_it_has_three_moves_on_middle_column() {
        let mut game = Game::default();

        _ = game.play(Player::X, Square::top_middle());
        _ = game.play(Player::O, Square::center_rigth());
        _ = game.play(Player::X, Square::center_middle());
        _ = game.play(Player::O, Square::top_rigth());

        assert!(matches!(
            game.play(Player::X, Square::bottom_middle()),
            Ok(Status::Win(Player::X))
        ));
    }

    #[test]
    fn declare_winner_if_it_has_three_moves_on_right_column() {
        let mut game = Game::default();

        _ = game.play(Player::X, Square::top_rigth());
        _ = game.play(Player::O, Square::center_left());
        _ = game.play(Player::X, Square::center_rigth());
        _ = game.play(Player::O, Square::center_middle());

        assert!(matches!(
            game.play(Player::X, Square::bottom_rigth()),
            Ok(Status::Win(Player::X))
        ));
    }
}
