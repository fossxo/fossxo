use amethyst::renderer::palette::Srgba;
use amethyst::{
    core::ecs, core::math::*, prelude::*, renderer::debug_drawing::DebugLinesComponent,
    window::ScreenDimensions,
};
use open_ttt_lib as ttt;

use crate::components;
use crate::math::*;
use crate::resources;

use super::environment::*;
use amethyst::core::Transform;
use amethyst::renderer::Camera;

#[derive(Default)]
pub struct DebugEnvironment {
    entities: Vec<ecs::Entity>,
}

impl Environment for DebugEnvironment {
    fn create(&mut self, world: &mut World, _options: EnvironmentOptions) {
        let (screen_w, screen_h) = {
            let screen_dimensions = world.read_resource::<ScreenDimensions>();
            (screen_dimensions.width(), screen_dimensions.height())
        };

        // Setup camera
        let screen_center = Point3::new(screen_w / 2.0, screen_h / 2.0, 1.0);
        let mut local_transform = Transform::default();
        local_transform.set_translation_xyz(screen_center.x, screen_center.y, 10.0);
        world
            .create_entity()
            .with(Camera::standard_2d(screen_w, screen_h))
            .with(local_transform)
            .build();

        let grid_size = screen_h * 0.8;
        let grid = resources::Grid::new(screen_center, grid_size);

        // Setup debug lines as a component and add lines to render axis&grid
        let mut debug_lines_component = DebugLinesComponent::new();
        for line in &grid.lines() {
            let color = Srgba::new(0.3, 0.3, 0.3, 1.0);
            debug_lines_component.add_line(line.start(), line.end(), color);
        }
        let lines_entity = world.create_entity().with(debug_lines_component).build();
        self.entities.push(lines_entity);
        world.insert(grid);
    }

    fn delete(&mut self, world: &mut World) {
        // Delete all entities.
        let _result = world.delete_entities(self.entities.as_slice());
    }

    fn add_mark(&mut self, world: &mut World, mark: &components::Mark) {
        let square = {
            let grid = world.read_resource::<resources::Grid>();
            grid.position_to_square(mark.position)
        };

        let color = Srgba::new(0.3, 0.3, 0.3, 1.0);
        let mut debug_lines_component = DebugLinesComponent::new();
        match mark.owner {
            components::Player::X => {
                let x_mark_square = Square::new(square.center_point(), square.size() * 0.8);
                debug_lines_component.add_line(
                    x_mark_square.top_left(),
                    x_mark_square.bottom_right(),
                    color,
                );
                debug_lines_component.add_line(
                    x_mark_square.bottom_left(),
                    x_mark_square.top_right(),
                    color,
                );
            }
            components::Player::O => debug_lines_component.add_circle_2d(
                square.center_point(),
                square.size() * 0.4,
                32,
                color,
            ),
        }

        let lines_entity = world.create_entity().with(debug_lines_component).build();
        self.entities.push(lines_entity);
    }

    fn game_over(&mut self, world: &mut World, _outcome: OutcomeAffinity) {
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

        if let Some(line) = winning_line {
            let color = Srgba::new(0.8, 0.3, 0.3, 1.0);
            let mut debug_lines_component = DebugLinesComponent::new();
            debug_lines_component.add_line(line.start(), line.end(), color);
            let lines_entity = world.create_entity().with(debug_lines_component).build();
            self.entities.push(lines_entity);
        }
    }
}
