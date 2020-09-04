use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, Write},
    renderer::debug_drawing::DebugLines,
};

use crate::components::*;
use crate::resources::*;

/// Responsible for drawing a debug boxes around hovered squares.
#[derive(SystemDesc)]
pub struct MouseHoverDebugBoxSystem;

impl<'a> System<'a> for MouseHoverDebugBoxSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'a, DebugLines>,
        ReadStorage<'a, MouseHoverDebugBox>,
        Read<'a, MousePosition>,
        Read<'a, Grid>,
        Read<'a, GameLogic>,
    );

    fn run(
        &mut self,
        (mut debug_lines, mouse_hover_debug_boxes, mouse_position, grid, game_logic): Self::SystemData,
    ) {
        for (component,) in (&mouse_hover_debug_boxes,).join() {
            let show_box = should_show_hover_box(component, &mouse_position, &game_logic);
            if show_box {
                let square = grid.position_to_square(mouse_position.grid);
                debug_lines.draw_rectangle(
                    square.bottom_left().xy(),
                    square.top_right().xy(),
                    square.center_point().z,
                    component.color,
                );
            }
        }
    }
}

// Indicates if the box should shown.
fn should_show_hover_box(
    mouse_hover_debug_box: &MouseHoverDebugBox,
    mouse_position: &MousePosition,
    game_logic: &GameLogic,
) -> bool {
    match mouse_hover_debug_box.visibility {
        MouseHoverVisibility::Hidden => false,
        MouseHoverVisibility::FreePositions => game_logic.game.can_move(mouse_position.grid),
        MouseHoverVisibility::AllPositions => game_logic.game.board().contains(mouse_position.grid),
    }
}
