//! Holds resources store global data that is not specific to any one entity.
use amethyst::core::math::*;
use open_ttt_lib as ttt;

use crate::components;
use crate::math::*;
use std::collections::{HashMap, HashSet};

/// Holds the current mouse position in various coordinate systems.
#[derive(Debug)]
pub struct MousePosition {
    /// The mouse location in screen coordinates.
    ///
    /// Note: this can be outside the bounds of the screen.
    pub screen: ScreenPoint,

    /// The mouse location in world coordinates.
    pub world: Point3<f32>,

    /// The mouse location in terms of the tic-tac-toe board.
    ///
    /// Note: if the mouse is outside the play area, these positions can be invalid.
    pub grid: ttt::game::Position,
}

impl Default for MousePosition {
    fn default() -> Self {
        Self {
            screen: ScreenPoint::new(0.0, 0.0),
            world: Point3::new(0.0, 0.0, 0.0),
            grid: ttt::game::Position { row: 0, column: 0 },
        }
    }
}

/// Provides access to the tic-tac-toe game logic.
///
/// This includes the current state of the game and the last time a move was made.
#[derive(Default)]
pub struct GameLogic {
    /// The tic-tac-toe game that contains the current game state.
    pub game: ttt::game::Game,

    /// Holds the time the game was last updated.
    pub last_move_time: std::time::Duration,
}

impl GameLogic {
    /// Helper function for knowing if it is the provided player's turn.
    pub fn is_players_move(&self, player: &components::Player) -> bool {
        match self.game.state() {
            ttt::game::State::PlayerXMove => *player == components::Player::X,
            ttt::game::State::PlayerOMove => *player == components::Player::O,
            _ => false,
        }
    }
}

// The size of tic-tac-toe boards that the grid works with.
const TTT_BOARD_SIZE: i32 = 3;

/// Represents the game's grid.
///
/// The grid is made up of a `#` mark. Various helper methods provide access to
/// the different points in the grid.
///
/// The grid assumes a square 3x3 game board.
#[derive(Clone)]
pub struct Grid {
    // The bottom left point of the grid. It is easier to do calculations from
    // the point than the center point.
    origin: Point3<f32>,
    // Grids are square. This is the length / width of the grid.
    size: f32,
}

impl Grid {
    /// Creates a grid centered around the indicated point and with the provided size.
    ///
    /// Grids are always square, thus the size sets the width and height of the grid.
    ///
    /// # Panics
    /// The size must be greater than 0.0.
    pub fn new(center_point: Point3<f32>, size: f32) -> Self {
        assert!(
            size > 0.0,
            "The size of the grid must be greater than zero."
        );

        let offset = size / 2.0;
        let origin = Point3::new(
            center_point.x - offset,
            center_point.y - offset,
            center_point.z,
        );
        Self { origin, size }
    }

    /// Gets the point at the center of the grid.
    pub fn center_point(&self) -> Point3<f32> {
        let offset = self.size / 2.0;
        Point3::new(
            self.origin.x + offset,
            self.origin.y + offset,
            self.origin.z,
        )
    }

    /// Gets the size of the grid.
    pub fn size(&self) -> f32 {
        self.size
    }

    /// Gets an iterator over all the squares in the grid.
    pub fn squares(&self) -> Squares {
        Squares {
            grid: &self,
            current_position: ttt::game::Position { row: 0, column: 0 },
            board_size: ttt::board::Size {
                rows: TTT_BOARD_SIZE,
                columns: TTT_BOARD_SIZE,
            },
        }
    }

    /// Gets the lines that make up the grid's hash marks.
    ///
    /// The vertical lines are first in the array followed by the horizontal lines.
    /// The points in the lines start at the bottom or left of the grid and end at
    /// the top or right of the grid.
    ///
    /// For example the array contains `[a-b, c-d, e-f, g-h]` as shown below:
    /// ```text
    ///       b   d
    ///       |   |
    ///  g ---+---+--- h
    ///       |   |
    ///  e ---+---+--- f
    ///       |   |
    ///       a   c
    /// ```
    pub fn lines(&self) -> [Line; 4] {
        let square_size = self.square_size();
        let a = Point3::new(self.origin.x + square_size, self.origin.y, self.origin.z);
        let b = Point3::new(
            self.origin.x + square_size,
            self.origin.y + self.size,
            self.origin.z,
        );
        let c = Point3::new(
            self.origin.x + square_size * 2.0,
            self.origin.y,
            self.origin.z,
        );
        let d = Point3::new(
            self.origin.x + square_size * 2.0,
            self.origin.y + self.size,
            self.origin.z,
        );
        let e = Point3::new(self.origin.x, self.origin.y + square_size, self.origin.z);
        let f = Point3::new(
            self.origin.x + self.size,
            self.origin.y + square_size,
            self.origin.z,
        );
        let g = Point3::new(
            self.origin.x,
            self.origin.y + square_size * 2.0,
            self.origin.z,
        );
        let h = Point3::new(
            self.origin.x + self.size,
            self.origin.y + square_size * 2.0,
            self.origin.z,
        );

        [
            Line::new(a, b),
            Line::new(c, d),
            Line::new(e, f),
            Line::new(g, h),
        ]
    }

    /// Returns a line that can be drawn through the provided winning positions.
    ///
    /// If multiple lines can be drawn through the positions, only one line is selected.
    /// The endpoints of the line are along the grid's edges. Lines start left or bottom
    /// of the grid. The top-left to bottom-right diagonal starts at the top left.
    ///
    /// `None` is returned if the winning positions is empty or a line cannot be
    /// drawn through them.
    pub fn wining_line(&self, winning_positions: &HashSet<ttt::game::Position>) -> Option<Line> {
        // Check the provided winning positions to see what diagonal, vertical, or horizontal
        // line to pick.
        if let Some(line) = self.bottom_left_to_top_right_winning_line(&winning_positions) {
            Some(line)
        } else if let Some(line) = self.top_left_to_bottom_right_winning_line(&winning_positions) {
            Some(line)
        } else if let Some(line) = self.vertical_or_horizontal_winning_line(&winning_positions) {
            Some(line)
        } else {
            None
        }
    }

    /// Converts the provided point to a grid position.
    ///
    /// The returned position can be outside the playable area of the board. E.g.
    /// it can have a negative row or column or very large row or column.
    ///
    /// If a point is picked that is directly between two squares the position
    /// returned could be for either square.
    pub fn point_to_position(&self, point: Point3<f32>) -> ttt::game::Position {
        let grid_point = (point - self.origin) / self.square_size();
        let row = grid_point.y.floor() as i32;
        let column = grid_point.x.floor() as i32;
        ttt::game::Position { row, column }
    }

    /// Gets a rectangle containing the bounds of the a specific position in the grid.
    ///
    /// This method does not check to see if the provided position is actually inside
    /// the grid; a position outside the grid results in a rectangle being returned
    /// that is also outside the bounds of the grid.
    pub fn position_to_square(&self, position: ttt::game::Position) -> Square {
        // Calculate the center point of the square based on the position, the square sizes, and
        // the grid origin.
        let square_size = self.square_size();
        let half_square_size = square_size / 2.0;
        let x = self.origin.x + square_size * position.column as f32 + half_square_size;
        let y = self.origin.y + square_size * position.row as f32 + half_square_size;
        let square_center_point = Point3::new(x, y, self.origin.z);

        Square::new(square_center_point, square_size)
    }

    // Gets the size of an and individual square.
    fn square_size(&self) -> f32 {
        self.size / 3.0
    }

    // Gets the bottom-left to top-right line.
    fn bottom_left_to_top_right_winning_line(
        &self,
        winning_positions: &HashSet<ttt::game::Position>,
    ) -> Option<Line> {
        for index in 0..TTT_BOARD_SIZE {
            let position = ttt::game::Position {
                row: index,
                column: index,
            };
            if !winning_positions.contains(&position) {
                return None;
            }
        }
        let start_square = self.position_to_square(ttt::game::Position { row: 0, column: 0 });
        let end_square = self.position_to_square(ttt::game::Position {
            row: TTT_BOARD_SIZE - 1,
            column: TTT_BOARD_SIZE - 1,
        });
        Some(Line::new(
            start_square.bottom_left(),
            end_square.top_right(),
        ))
    }

    // Gets the top-left to bottom-right line.
    fn top_left_to_bottom_right_winning_line(
        &self,
        winning_positions: &HashSet<ttt::game::Position>,
    ) -> Option<Line> {
        for index in 0..TTT_BOARD_SIZE {
            let position = ttt::game::Position {
                row: TTT_BOARD_SIZE - index - 1,
                column: index,
            };
            if !winning_positions.contains(&position) {
                return None;
            }
        }
        let start_square = self.position_to_square(ttt::game::Position {
            row: TTT_BOARD_SIZE - 1,
            column: 0,
        });
        let end_square = self.position_to_square(ttt::game::Position {
            row: 0,
            column: TTT_BOARD_SIZE - 1,
        });
        Some(Line::new(
            start_square.top_left(),
            end_square.bottom_right(),
        ))
    }

    fn vertical_or_horizontal_winning_line(
        &self,
        winning_positions: &HashSet<ttt::game::Position>,
    ) -> Option<Line> {
        // Build a mapping of rows and columns to how many times the row or column
        // appeared in the winning positions. If the same row / column appears
        // the same number as the board size then we know we can draw a line
        // through it.
        let mut rows_found = HashMap::new();
        let mut columns_found = HashMap::new();
        for position in winning_positions {
            *rows_found.entry(position.row).or_insert(0) += 1;
            *columns_found.entry(position.column).or_insert(0) += 1;
        }

        // Using the rows and columns found, build a horizontal or vertical line for the first one
        // that has the same number (or more) as the board size. None is returned if a line could
        // not be built.
        if let Some((column, _)) = columns_found
            .iter()
            .find(|(_col, num)| **num >= TTT_BOARD_SIZE)
        {
            let start_square = self.position_to_square(ttt::game::Position {
                row: 0,
                column: *column,
            });
            let end_square = self.position_to_square(ttt::game::Position {
                row: TTT_BOARD_SIZE - 1,
                column: *column,
            });
            Some(Line::new(start_square.bottom(), end_square.top()))
        } else if let Some((row, _)) = rows_found
            .iter()
            .find(|(_row, num)| **num >= TTT_BOARD_SIZE)
        {
            let start_square = self.position_to_square(ttt::game::Position {
                row: *row,
                column: 0,
            });
            let end_square = self.position_to_square(ttt::game::Position {
                row: *row,
                column: TTT_BOARD_SIZE - 1,
            });
            Some(Line::new(start_square.left(), end_square.right()))
        } else {
            None
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(Point3::new(0.0, 0.0, 0.0), 1.0)
    }
}

/// An iterator over the squares that are inside the `Grid`.
///
/// Squares always start at position (0, 0) then increase first by columns then
/// by rows until the end of the grid is reached.
pub struct Squares<'a> {
    grid: &'a Grid,
    current_position: ttt::game::Position,
    board_size: ttt::board::Size,
}

impl Iterator for Squares<'_> {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        // Check to see if the iterator has already been exhausted.
        if self.current_position.row >= self.board_size.rows {
            None
        } else {
            let position = self.current_position;
            // Advance the position checking to see if the column needs to wrap around.
            self.current_position.column += 1;
            if self.current_position.column >= self.board_size.columns {
                self.current_position.row += 1;
                self.current_position.column = 0;
            }

            let square = self.grid.position_to_square(position);
            Some(square)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use approx::assert_ulps_eq;

    #[test]
    #[should_panic]
    fn grid_new_when_size_zero_should_panic() {
        let center_point = Point3::new(1.0, 2.0, 3.0);
        let size = 0.0;

        let _grid = Grid::new(center_point, size);
    }

    #[test]
    fn grid_new_should_set_properties() {
        let center_point = Point3::new(1.0, 2.0, 3.0);
        let size = 4.0;

        let grid = Grid::new(center_point, size);

        assert_ulps_eq!(grid.center_point(), center_point);
        assert_ulps_eq!(grid.size(), size);
    }

    #[test]
    fn grid_squares_should_contain_same_number_of_squares_as_game() {
        let center_point = Point3::new(0.0, 0.0, 0.0);
        let size = 3.0;
        let grid = Grid::new(center_point, size);
        // The game provides access to the board the grid represents.
        let game = ttt::game::Game::new();
        let expected_num_squares = game.board().iter().count();

        let actual_num_squares = grid.squares().count();

        assert_eq!(actual_num_squares, expected_num_squares);
    }

    #[test]
    fn grid_squares_should_starts_at_row_0_col_0() {
        let center_point = Point3::new(0.0, 0.0, 0.0);
        let size = 3.0;
        let grid = Grid::new(center_point, size);
        let position = ttt::game::Position { row: 0, column: 0 };
        let expected_square = grid.position_to_square(position);

        let actual_square = grid.squares().next().unwrap();

        assert_eq!(actual_square, expected_square);
    }

    #[test]
    fn grid_squares_should_ends_at_row_2_col_2() {
        let center_point = Point3::new(0.0, 0.0, 0.0);
        let size = 3.0;
        let grid = Grid::new(center_point, size);
        let position = ttt::game::Position { row: 2, column: 2 };
        let expected_square = grid.position_to_square(position);

        let actual_square = grid.squares().last().unwrap();

        assert_eq!(actual_square, expected_square);
    }

    #[test]
    fn grid_squares_should_increment_by_columns_first() {
        let center_point = Point3::new(0.0, 0.0, 0.0);
        let size = 3.0;
        let grid = Grid::new(center_point, size);
        let position = ttt::game::Position { row: 0, column: 1 };
        let expected_square = grid.position_to_square(position);

        // Skip the first square to get to the second one.
        let actual_square = grid.squares().skip(1).next().unwrap();

        assert_eq!(actual_square, expected_square);
    }

    #[test]
    fn grid_lines_should_be_same_size_as_grid() {
        let center_point = Point3::new(0.0, 0.0, 0.0);
        let size = 3.0;
        let grid = Grid::new(center_point, size);

        let line = grid.lines()[0];
        let actual_line_length = distance(&line.start(), &line.end());

        assert_ulps_eq!(actual_line_length, size);
    }

    #[test]
    fn grid_lines_should_provide_vertical_then_horizontal_lines() {
        let center_point = Point3::new(0.0, 0.0, 0.0);
        let size = 3.0;
        let grid = Grid::new(center_point, size);

        let lines = grid.lines();

        // The vertical lines should have the same starting and ending Y value.
        assert_ulps_eq!(lines[0].start().x, lines[0].end().x);
        assert_ulps_eq!(lines[1].start().x, lines[1].end().x);
        // And the horizontal lines have the same starting and ending X value.
        assert_ulps_eq!(lines[2].start().y, lines[2].end().y);
        assert_ulps_eq!(lines[3].start().y, lines[3].end().y);
    }

    #[test]
    fn grid_lines_vertical_should_start_at_bottom() {
        let center_point = Point3::new(0.0, 0.0, 0.0);
        let size = 4.0;
        // Because the center point is zero, we know where the line should start
        // based on the size.
        let expected_bottom = -size / 2.0;
        let grid = Grid::new(center_point, size);

        let lines = grid.lines();

        // The vertical lines should have the same starting and ending X value.
        assert_ulps_eq!(lines[0].start().y, expected_bottom);
        assert_ulps_eq!(lines[1].start().y, expected_bottom);
    }

    #[test]
    fn grid_lines_horizontal_should_start_at_left_side() {
        let center_point = Point3::new(0.0, 0.0, 0.0);
        let size = 4.0;
        // Because the center point is zero, we know where the line should start
        // based on the size.
        let expected_left = -size / 2.0;
        let grid = Grid::new(center_point, size);

        let lines = grid.lines();

        // The vertical lines should have the same starting and ending X value.
        assert_ulps_eq!(lines[2].start().x, expected_left);
        assert_ulps_eq!(lines[3].start().x, expected_left);
    }

    #[test]
    fn grid_wining_line_when_empty_positions_should_return_none() {
        let center_point = Point3::new(0.0, 0.0, 0.0);
        let size = 3.0;
        let grid = Grid::new(center_point, size);
        let winning_positions = HashSet::new();

        let wining_line = grid.wining_line(&winning_positions);

        assert_eq!(wining_line, None);
    }

    #[test]
    fn grid_wining_line_when_cannot_draw_line_through_positions_should_return_none() {
        let center_point = Point3::new(0.0, 0.0, 0.0);
        let size = 3.0;
        let grid = Grid::new(center_point, size);
        let mut winning_positions = HashSet::new();
        winning_positions.insert(ttt::game::Position { row: 0, column: 0 });
        winning_positions.insert(ttt::game::Position { row: 1, column: 2 });
        winning_positions.insert(ttt::game::Position { row: 2, column: 0 });

        let wining_line = grid.wining_line(&winning_positions);

        assert_eq!(wining_line, None);
    }

    #[test]
    fn grid_wining_line_when_vertical_positions_should_start_at_bottom() {
        // To make calculations easier, place the grid's bottom left corner at (0, 0, 0).
        let size = 3.0;
        let center_point = Point3::new(size / 2.0, size / 2.0, 0.0);
        let grid = Grid::new(center_point, size);
        let mut winning_positions = HashSet::new();
        // Using the middle column means the line should go through the grid's center point.
        winning_positions.insert(ttt::game::Position { row: 2, column: 1 });
        winning_positions.insert(ttt::game::Position { row: 1, column: 1 });
        winning_positions.insert(ttt::game::Position { row: 0, column: 1 });
        let expected_start_point = Point3::new(center_point.x, 0.0, 0.0);
        let expected_end_point = Point3::new(center_point.x, size, 0.0);

        let actual_line = grid.wining_line(&winning_positions).unwrap();

        assert_ulps_eq!(actual_line.start(), expected_start_point);
        assert_ulps_eq!(actual_line.end(), expected_end_point);
    }

    #[test]
    fn grid_wining_line_when_horizontal_positions_should_start_at_left() {
        // To make calculations easier, place the grid's bottom left corner at (0, 0, 0).
        let size = 3.0;
        let center_point = Point3::new(size / 2.0, size / 2.0, 0.0);
        let grid = Grid::new(center_point, size);
        let mut winning_positions = HashSet::new();
        // Using the middle row means the line should go through the grid's center point.
        winning_positions.insert(ttt::game::Position { row: 1, column: 0 });
        winning_positions.insert(ttt::game::Position { row: 1, column: 1 });
        winning_positions.insert(ttt::game::Position { row: 1, column: 2 });
        let expected_start_point = Point3::new(0.0, center_point.y, 0.0);
        let expected_end_point = Point3::new(size, center_point.y, 0.0);

        let actual_line = grid.wining_line(&winning_positions).unwrap();

        assert_ulps_eq!(actual_line.start(), expected_start_point);
        assert_ulps_eq!(actual_line.end(), expected_end_point);
    }

    #[test]
    fn grid_wining_line_when_bottom_left_to_top_right_diagonal_should_start_at_bottom_left() {
        // To make calculations easier, place the grid's bottom left corner at (0, 0, 0).
        let size = 3.0;
        let center_point = Point3::new(size / 2.0, size / 2.0, 0.0);
        let grid = Grid::new(center_point, size);
        let mut winning_positions = HashSet::new();
        winning_positions.insert(ttt::game::Position { row: 0, column: 0 });
        winning_positions.insert(ttt::game::Position { row: 1, column: 1 });
        winning_positions.insert(ttt::game::Position { row: 2, column: 2 });
        let expected_start_point = Point3::new(0.0, 0.0, 0.0);
        let expected_end_point = Point3::new(size, size, 0.0);

        let actual_line = grid.wining_line(&winning_positions).unwrap();

        assert_ulps_eq!(actual_line.start(), expected_start_point);
        assert_ulps_eq!(actual_line.end(), expected_end_point);
    }

    #[test]
    fn grid_wining_line_when_top_left_to_bottom_right_diagonal_should_start_at_top_left() {
        // To make calculations easier, place the grid's bottom left corner at (0, 0, 0).
        let size = 3.0;
        let center_point = Point3::new(size / 2.0, size / 2.0, 0.0);
        let grid = Grid::new(center_point, size);
        let mut winning_positions = HashSet::new();
        winning_positions.insert(ttt::game::Position { row: 2, column: 0 });
        winning_positions.insert(ttt::game::Position { row: 1, column: 1 });
        winning_positions.insert(ttt::game::Position { row: 0, column: 2 });
        let expected_start_point = Point3::new(0.0, size, 0.0);
        let expected_end_point = Point3::new(size, 0.0, 0.0);

        let actual_line = grid.wining_line(&winning_positions).unwrap();

        assert_ulps_eq!(actual_line.start(), expected_start_point);
        assert_ulps_eq!(actual_line.end(), expected_end_point);
    }

    #[test]
    fn grid_wining_line_when_multiple_winning_lines_should_provide_line() {
        // To make calculations easier, place the grid's bottom left corner at (0, 0, 0).
        let size = 3.0;
        let center_point = Point3::new(0.0, 0.0, 0.0);
        let grid = Grid::new(center_point, size);
        let mut winning_positions = HashSet::new();
        winning_positions.insert(ttt::game::Position { row: 2, column: 0 });
        winning_positions.insert(ttt::game::Position { row: 1, column: 1 });
        winning_positions.insert(ttt::game::Position { row: 0, column: 2 });
        winning_positions.insert(ttt::game::Position { row: 0, column: 1 });
        winning_positions.insert(ttt::game::Position { row: 0, column: 0 });

        let actual_line = grid.wining_line(&winning_positions);

        // Ensure some line was returned, however we don't care about the details.
        assert_ne!(actual_line, None);
    }

    #[test]
    fn grid_point_to_position_center_point_should_be_at_row_1_column_1() {
        let center_point = Point3::new(0.0, 0.0, 0.0);
        let size = 3.0;
        let grid = Grid::new(center_point, size);
        let expected_position = ttt::game::Position { row: 1, column: 1 };

        let actual_position = grid.point_to_position(center_point);

        assert_eq!(actual_position, expected_position);
    }

    #[test]
    fn grid_point_to_position_right_of_center_should_be_at_row_1_column_2() {
        let center_point = Point3::new(0.0, 0.0, 0.0);
        let size = 3.0;
        let grid = Grid::new(center_point, size);
        // Get the point to the right side of the grid. We ensure the point is not
        // directly on the line as the grid is free to place that point in the next
        // square over.
        let right_of_center = Point3::new(1.5 - 0.0125, 0.0, 0.0);
        let expected_position = ttt::game::Position { row: 1, column: 2 };

        let actual_position = grid.point_to_position(right_of_center);

        assert_eq!(actual_position, expected_position);
    }

    #[test]
    fn grid_position_to_square_when_row_0_col_0_should_be_bottom_left_square() {
        let center_point = Point3::new(0.0, 0.0, 0.0);
        let size = 3.0;
        let grid = Grid::new(center_point, size);
        let position = ttt::game::Position { row: 0, column: 0 };
        let expected_square = grid.squares().next().unwrap();

        let actual_square = grid.position_to_square(position);

        assert_ulps_eq!(actual_square.center_point(), expected_square.center_point());
        assert_ulps_eq!(actual_square.size(), expected_square.size());
    }
}
