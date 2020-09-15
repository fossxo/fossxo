use amethyst::ecs;
use amethyst::prelude::*;
use contracts::*;

use super::*;
use crate::{components, events};
use amethyst::ui::{Anchor, UiButton, UiButtonBuilder, UiEventType, UiLabelBuilder};

pub struct GameControls<TData, TReturn = ()> {
    owned_entities: Vec<ecs::Entity>,
    observers: EntityObservers<TData, TReturn>,
    game_over_button: Option<UiButton>,
}

impl<TData, TReturn> GameControls<TData, TReturn> {
    pub fn new() -> Self {
        Self {
            owned_entities: Vec::new(),
            observers: EntityObservers::new(),
            game_over_button: None,
        }
    }

    /// Deletes all widget entities from the menu.
    ///
    /// Any previously created widgets are invalid and should no longer be used.
    #[post(self.owned_entities.is_empty())]
    #[post(self.game_over_button.is_none())]
    pub fn delete(&mut self, world: &mut ecs::World) {
        self.hide_game_over_button(world);

        world
            .delete_entities(self.owned_entities.as_slice())
            .expect("Unable to delete game control entities.");

        self.owned_entities.clear();
    }

    /// Handles the provided UI event.
    ///
    /// The callback associated with the event, if any, is returned for the caller to invoke.
    pub fn handle_ui_event(
        &mut self,
        _world: &mut ecs::World,
        ui_event: &events::UiEvent,
    ) -> Option<&fn(&mut TData, &mut ecs::World) -> TReturn> {
        match ui_event.event_type {
            UiEventType::Click => self.observers.get(ui_event.target),
            _ => None,
        }
    }

    /// Sets menu button.
    pub fn set_menu_button(
        &mut self,
        world: &mut ecs::World,
        on_press: fn(&mut TData, &mut ecs::World) -> TReturn,
    ) {
        let style = world.read_resource::<Style>();
        let size = style.hamburger_button.size;
        let (_button_id, button) = UiButtonBuilder::<(), u32>::new("")
            .with_anchor(Anchor::TopRight)
            .with_position(-size / 2.0, -size / 2.0)
            .with_size(size, size)
            .with_image(style.hamburger_button.normal.clone())
            .with_hover_image(style.hamburger_button.hover.clone())
            .with_press_image(style.hamburger_button.press.clone())
            .build_from_world(&world);

        self.add_owned_button(&button);
        self.observers.add(button.image_entity, on_press);
    }

    /// Show's the game over button.
    pub fn show_game_over_button(
        &mut self,
        world: &mut ecs::World,
        on_press: fn(&mut TData, &mut ecs::World) -> TReturn,
    ) {
        // Remove any old game over buttons.
        self.hide_game_over_button(world);

        let style = world.read_resource::<Style>();

        let (_button_id, button) = UiButtonBuilder::<(), u32>::new("Play Again?")
            .with_font(style.button.text.font.clone())
            .with_font_size(style.button.text.font_size)
            .with_text_color(style.button.text.color)
            .with_image(style.button.normal.clone())
            .with_hover_image(style.button.hover.clone())
            .with_press_image(style.button.press.clone())
            .with_anchor(Anchor::BottomMiddle)
            .with_position(0.0, 50.0)
            .with_size(style.button.width, style.button.height)
            .build_from_world(&world);

        self.observers.add(button.image_entity, on_press);
        self.game_over_button = Some(button);
    }

    /// Hide's the game over button.
    pub fn hide_game_over_button(&mut self, world: &mut ecs::World) {
        if let Some(button) = self.game_over_button.take() {
            self.observers.remove(button.image_entity);
            world
                .delete_entity(button.text_entity)
                .expect("Unable to delete game over button.");
            world
                .delete_entity(button.image_entity)
                .expect("Unable to delete game over button.");
        }
    }

    /// Sets the status text along with extra information.
    pub fn set_status(&mut self, world: &mut ecs::World, extra_information: Vec<String>) {
        let (_id, label) = UiLabelBuilder::<u32>::new("")
            .with_anchor(Anchor::TopLeft)
            .with_align(Anchor::MiddleLeft)
            .with_size(600.0, 84.0)
            // The position is slightly past the 1/2 way part so the text is not
            // right at the edge of the screen.
            .with_position(310.0, -20.0)
            .with_font_size(20.0)
            .with_text_color([1.0, 1.0, 1.0, 1.0])
            .build_from_world(world);
        self.owned_entities.push(label.text_entity);

        // Create the game state entity that gets updated to reflect the state of the game.
        self.owned_entities.push(
            world
                .create_entity()
                .with(components::GameStateText {
                    ui_text: label.text_entity,
                    extra_information,
                })
                .build(),
        );
    }

    // Adds the entities in the provided button to the list of owned entities.
    fn add_owned_button(&mut self, button: &UiButton) {
        self.owned_entities.push(button.text_entity);
        self.owned_entities.push(button.image_entity);
    }
}
