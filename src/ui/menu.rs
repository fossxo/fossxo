use super::*;
use crate::events;

use amethyst::ui::{Anchor, LineMode, UiButton, UiButtonBuilder, UiEventType, UiText, UiTransform};
use amethyst::{ecs, prelude::*};

use contracts::*;

const CLOSE_TAB_ORDER: u32 = 100;
// Amount of space the separator takes.
const SEPARATOR_HEIGHT: f32 = 20.0;
// Spacing between elements.
const MARGIN: f32 = 10.0;

/// Allows creating Menus related widgets, provides UI event handling logic, and
/// holds the underlying entities.
pub struct Menu<TData, TReturn = ()> {
    owned_entities: Vec<ecs::Entity>,
    observers: EntityObservers<TData, TReturn>,
    next_y_offset: f32,
    next_tab_order: u32,
}

impl<TData, TReturn> Menu<TData, TReturn> {
    /// Creates a new Menu.
    pub fn new() -> Self {
        Self {
            owned_entities: Vec::new(),
            observers: EntityObservers::new(),
            next_y_offset: 150.0,
            next_tab_order: 1,
        }
    }

    /// Deletes all widget entities from the menu.
    ///
    /// Any previously created widgets are invalid and should no longer be used.
    #[post(self.owned_entities.is_empty())]
    pub fn delete(&mut self, world: &mut ecs::World) {
        world
            .delete_entities(self.owned_entities.as_slice())
            .expect("Unable to delete menu entities.");

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

    /// Sets the menu's title text.
    pub fn set_title(&mut self, world: &mut ecs::World, text: &str) {
        let title_style = {
            let style = world.read_resource::<Style>();
            style.title_text.clone()
        };

        let text = UiText::new(
            title_style.font,
            text.to_string(),
            title_style.color,
            title_style.font_size,
            LineMode::Single,
            Anchor::Middle,
        );

        let transform = UiTransform::new(
            "id".to_string(),
            Anchor::TopMiddle,
            Anchor::Middle,
            0.0,
            -40.0,
            1.0,
            800.0,
            60.0,
        );

        let title_entity = world.create_entity().with(text).with(transform).build();
        self.owned_entities.push(title_entity);
    }

    /// Sets the menu close button.
    pub fn set_close_button(
        &mut self,
        world: &mut ecs::World,
        text: &str,
        on_close: fn(&mut TData, &mut ecs::World) -> TReturn,
    ) {
        let style = world.read_resource::<Style>();

        let (_button_id, button) = initialize_button(text, &style)
            .with_anchor(Anchor::BottomMiddle)
            .with_position(0.0, 50.0)
            .with_tab_order(CLOSE_TAB_ORDER)
            .build_from_world(&world);

        self.add_owned_button(&button);
        self.observers.add(button.image_entity, on_close);
    }

    /// Adds a button to the menu.
    ///
    /// The order in which this method is called determines the order buttons appear in the menu.
    pub fn add_button(
        &mut self,
        world: &mut ecs::World,
        text: &str,
        on_press: fn(&mut TData, &mut ecs::World) -> TReturn,
    ) {
        let style = world.read_resource::<Style>();

        let (_button_id, button) = initialize_button(text, &style)
            .with_position(0.0, self.next_y_offset)
            .with_tab_order(self.next_tab_order)
            .build_from_world(&world);

        self.next_y_offset -= style.button.height + MARGIN;
        self.next_tab_order += 1;
        self.add_owned_button(&button);
        self.observers.add(button.image_entity, on_press);
    }

    /// Adds a separator between the current content.
    pub fn add_separator(&mut self, _world: &mut ecs::World) {
        self.next_y_offset -= SEPARATOR_HEIGHT;
    }

    // Adds the entities in the provided button to the list of owned entities.
    fn add_owned_button(&mut self, button: &UiButton) {
        self.owned_entities.push(button.text_entity);
        self.owned_entities.push(button.image_entity);
    }
}

// Creates a button builder with the common properties filled in.
fn initialize_button(text: &str, style: &Style) -> UiButtonBuilder<(), u32> {
    // let button_style = &style.button;
    UiButtonBuilder::<(), u32>::new(text)
        .with_font(style.button.text.font.clone())
        .with_font_size(style.button.text.font_size)
        .with_text_color(style.button.text.color)
        .with_anchor(Anchor::Middle)
        .with_size(style.button.width, style.button.height)
        .with_image(style.button.normal.clone())
        .with_hover_image(style.button.hover.clone())
        .with_press_image(style.button.press.clone())
}
