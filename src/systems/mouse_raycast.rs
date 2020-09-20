use amethyst::{
    core::{math::*, Transform},
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, Read, ReadExpect, ReadStorage, System, SystemData, Write},
    input::InputHandler,
    renderer::{ActiveCamera, Camera},
    window::ScreenDimensions,
};

use crate::events::InputBindingTypes;
use crate::math::*;
use crate::resources::*;

/// Responsible for converting the mouse's position from screen to world
/// coordinates and grid position.
#[derive(SystemDesc)]
pub struct MouseRaycastSystem;

impl<'a> System<'a> for MouseRaycastSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'a, MousePosition>,
        Entities<'a>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Transform>,
        Read<'a, ActiveCamera>,
        ReadExpect<'a, ScreenDimensions>,
        Read<'a, InputHandler<InputBindingTypes>>,
        Read<'a, Grid>,
    );

    fn run(
        &mut self,
        (
            mut mouse_position,
            entities,
            cameras,
            transforms,
            active_camera,
            screen_dimensions,
            input,
            grid,
        ): Self::SystemData,
    ) {
        // Get the mouse position if its available.
        if let Some(mouse) = input.mouse_position() {
            // Get the active camera if it is spawned and .
            let mut camera_join = (&cameras, &transforms).join();
            if let Some((camera, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                update_mouse_position(
                    &mut mouse_position,
                    &screen_dimensions,
                    &grid,
                    mouse,
                    camera,
                    camera_transform,
                );
            }
        }
    }
}

// Updates the provided mouse position using available data.
fn update_mouse_position(
    mouse_position: &mut MousePosition,
    screen_dimensions: &ScreenDimensions,
    grid: &Grid,
    mouse: (f32, f32),
    camera: &Camera,
    camera_transform: &Transform,
) {
    let screen_point = ScreenPoint::from(mouse);
    let world_point = camera.screen_to_world_point(
        Point3::new(screen_point.x, screen_point.y, 0.0),
        screen_dimensions.diagonal(),
        &camera_transform,
    );
    let grid_position = grid.point_to_position(world_point);

    mouse_position.screen = screen_point;
    mouse_position.world = world_point;
    mouse_position.grid = grid_position;
}
