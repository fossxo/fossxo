use amethyst::{
    core::ecs,
    prelude::*,
    renderer::{debug_drawing::DebugLinesComponent, palette::Srgba},
};
use contracts::*;
use open_ttt_lib as ttt;

use crate::components;
use crate::math::*;
use crate::resources;

use super::environment::*;

/// Holds options related to showing the debug environment.
///
/// Note: You can use the structure update syntax to quickly set the options of interest.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct DebugOptions {
    /// Shows the interior lines that form the grid, e.g. the # shape.
    pub grid: bool,
    /// Shows the outer border of the grid.
    pub border: bool,
    /// Show the X and O marks.
    pub marks: bool,
    /// Show the line drawn through the the winning marks.
    pub win_line: bool,
    /// Shows the square center point positions.
    pub center_points: bool,
    /// How to highlight the square currently being hovered over by the mouse.
    pub mouse_hover: components::MouseHoverVisibility,
}

impl DebugOptions {
    /// Enables all available debugging options.
    pub fn enable_all() -> Self {
        Self {
            grid: true,
            border: true,
            marks: true,
            win_line: true,
            center_points: true,
            mouse_hover: components::MouseHoverVisibility::AllPositions,
        }
    }

    /// Disables all debugging options.
    pub fn disable_all() -> Self {
        Self {
            grid: false,
            border: false,
            marks: false,
            win_line: false,
            center_points: false,
            mouse_hover: components::MouseHoverVisibility::Hidden,
        }
    }
}

impl Default for DebugOptions {
    fn default() -> Self {
        Self {
            grid: true,
            border: false,
            marks: true,
            win_line: true,
            center_points: false,
            mouse_hover: components::MouseHoverVisibility::FreePositions,
        }
    }
}

// The size of the debug environment marks, relative to the square size.
const MARK_SIZE_FACTOR: f32 = 0.8;
// Size of the center point graphic,  relative to the square size.
const CENTER_POINT_SIZE_FACTOR: f32 = 0.0625;

#[derive(Default)]
pub struct DebugEnvironment {
    // The options that control how how the environment is shown.
    options: DebugOptions,
    // The main color to use when drawing the environment.
    color: Srgba,
    // All the entities owned by this environment.
    entities: Vec<ecs::Entity>,
}

impl DebugEnvironment {
    /// Creates a new Debug environment using the provided options.
    pub fn new(options: DebugOptions) -> Self {
        Self {
            options,
            color: Srgba::new(0.3, 0.3, 0.3, 1.0),
            entities: Vec::new(),
        }
    }

    // Adds the grid lines, e.g. the # shape.
    fn add_grid(&self, debug_lines: &mut DebugLinesComponent, grid: &resources::Grid) {
        for line in &grid.lines() {
            debug_lines.add_line(line.start(), line.end(), self.color);
        }
    }

    // Adds a border around the grid.
    fn add_border(&self, debug_lines: &mut DebugLinesComponent, grid: &resources::Grid) {
        let grid_border = Square::new(grid.center_point(), grid.size());
        debug_lines.add_rectangle_2d(
            grid_border.bottom_left().xy(),
            grid_border.top_right().xy(),
            grid.center_point().z,
            self.color,
        );
    }

    // Shows the square center point positions.
    fn add_center_points(&self, debug_lines: &mut DebugLinesComponent, grid: &resources::Grid) {
        for square in grid.squares() {
            let length = square.size() * CENTER_POINT_SIZE_FACTOR;
            let center_point_square = Square::new(square.center_point(), length);
            debug_lines.add_line(
                center_point_square.bottom(),
                center_point_square.top(),
                self.color,
            );
            debug_lines.add_line(
                center_point_square.left(),
                center_point_square.right(),
                self.color,
            );
        }
    }

    // Add an X mark to the debug lines component in the indicated square.
    fn add_x_mark(&self, debug_lines: &mut DebugLinesComponent, square: &Square) {
        let x_mark_square = Square::new(square.center_point(), square.size() * MARK_SIZE_FACTOR);
        debug_lines.add_line(
            x_mark_square.top_left(),
            x_mark_square.bottom_right(),
            self.color,
        );
        debug_lines.add_line(
            x_mark_square.bottom_left(),
            x_mark_square.top_right(),
            self.color,
        );
    }

    // Add an O mark to the debug lines component in the indicated square.
    fn add_o_mark(&self, debug_lines: &mut DebugLinesComponent, square: &Square) {
        let radius = square.size() * MARK_SIZE_FACTOR / 2.0;
        let points = 32;
        debug_lines.add_circle_2d(square.center_point(), radius, points, self.color);
    }
}

impl Environment for DebugEnvironment {
    fn create(&mut self, world: &mut World) {
        // Get a copy of the grid resource so we know where to place the various lines.
        let grid = {
            let grid_resource = world.read_resource::<resources::Grid>();
            *grid_resource
        };

        // Create the lines for the grid, border, and center points.
        let mut debug_lines_component = DebugLinesComponent::new();
        if self.options.grid {
            self.add_grid(&mut debug_lines_component, &grid);
        }

        if self.options.border {
            self.add_border(&mut debug_lines_component, &grid);
        }

        if self.options.center_points {
            self.add_center_points(&mut debug_lines_component, &grid);
        }
        self.entities
            .push(world.create_entity().with(debug_lines_component).build());

        // TODO: Add any existing marks
        // TODO: Add any existing win lines

        // Create the mouse hover box.
        if self.options.mouse_hover != components::MouseHoverVisibility::Hidden {
            let mouse_hover_debug_box = components::MouseHoverDebugBox {
                color: self.color,
                visibility: self.options.mouse_hover,
            };
            self.entities
                .push(world.create_entity().with(mouse_hover_debug_box).build());
        }
    }

    #[post(self.entities.len() == 0)]
    fn delete(&mut self, world: &mut World) {
        // Delete all entities.
        let result = world.delete_entities(self.entities.as_slice());
        if let Err(e) = result {
            log::error!("Unable to delete entities from environment. Details: {}", e);
        }
        self.entities.clear();
    }

    fn add_mark(&mut self, world: &mut World, mark: &components::Mark) {
        if !self.options.marks {
            return;
        }

        // Determine where to place the mark.
        let square_for_mark = {
            let grid = world.read_resource::<resources::Grid>();
            grid.position_to_square(mark.position)
        };

        // Add the corresponding mark.
        let mut debug_lines_component = DebugLinesComponent::new();
        match mark.owner {
            components::Player::X => self.add_x_mark(&mut debug_lines_component, &square_for_mark),
            components::Player::O => self.add_o_mark(&mut debug_lines_component, &square_for_mark),
        }

        self.entities
            .push(world.create_entity().with(debug_lines_component).build());
    }

    fn game_over(&mut self, world: &mut World, _outcome: OutcomeAffinity) {
        if !self.options.win_line {
            return;
        }

        // Get the line that goes through the winning marks.
        let winning_line = {
            let game_logic = world.read_resource::<resources::GameLogic>();
            let grid = world.read_resource::<resources::Grid>();
            match game_logic.game.state() {
                ttt::game::State::PlayerXWin(winning_positions) => {
                    grid.wining_line(&winning_positions)
                }
                ttt::game::State::PlayerOWin(winning_positions) => {
                    grid.wining_line(&winning_positions)
                }
                _ => None,
            }
        };

        // Add the line, if one was found, through the marks.
        if let Some(line) = winning_line {
            let mut debug_lines_component = DebugLinesComponent::new();
            debug_lines_component.add_line(line.start(), line.end(), self.color);
            self.entities
                .push(world.create_entity().with(debug_lines_component).build());
        }
    }

    fn is_alive(&self) -> bool {
        !self.entities.is_empty()
    }
}
