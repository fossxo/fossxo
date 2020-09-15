//! Holds UI styles

use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{ImageFormat, Texture};
use amethyst::ui::{get_default_font, FontAsset, FontHandle, UiImage};
use std::path::Path;

/// Loads the UI style and places it in the world.
pub fn load_style(world: &mut World) {
    let white = [1.0, 1.0, 1.0, 1.0];

    let font = {
        let loader = world.read_resource::<Loader>();
        let storage = world.read_resource::<AssetStorage<FontAsset>>();
        get_default_font(&loader, &storage)
    };

    let title_text = TextStyle {
        font: font.clone(),
        font_size: 75.0,
        color: white,
    };

    let label = TextStyle {
        font,
        font_size: 50.0,
        color: white,
    };

    let menu = MenuStyle {
        background: UiImage::SolidColor([0.03, 0.03, 0.03, 1.0]),
    };

    let button = ButtonStyle {
        text: label.clone(),
        width: 600.0,
        height: 75.0,
        normal: UiImage::SolidColor([0.2, 0.2, 0.2, 1.0]),
        hover: UiImage::SolidColor([0.1, 0.1, 0.1, 1.0]),
        press: UiImage::SolidColor([0.15, 0.15, 0.15, 1.0]),
    };

    let hamburger_icon_hover = load_texture(world, "hamburger-icon-hover.png");
    let hamburger_button = HamburgerButtonStyle {
        size: 48.0,
        normal: UiImage::Texture(load_texture(world, "hamburger-icon-normal.png")),
        hover: UiImage::Texture(hamburger_icon_hover.clone()),
        press: UiImage::Texture(hamburger_icon_hover),
    };

    let ui_style = Style {
        button,
        hamburger_button,
        title_text,
        menu,
        paragraph: label.clone(),
        label,
    };

    world.insert(ui_style);
}

fn load_texture(world: &World, name: &str) -> Handle<Texture> {
    let path = Path::new("textures").join(name);
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
        path.to_string_lossy(),
        ImageFormat::default(),
        (),
        &texture_storage,
    )
}

#[derive(Clone)]
pub(super) struct Style {
    pub button: ButtonStyle,
    pub hamburger_button: HamburgerButtonStyle,
    pub title_text: TextStyle,
    pub menu: MenuStyle,
    pub paragraph: TextStyle,
    pub label: TextStyle,
}

#[derive(Clone)]
pub(super) struct ButtonStyle {
    pub text: TextStyle,
    pub width: f32,
    pub height: f32,
    pub normal: UiImage,
    pub hover: UiImage,
    pub press: UiImage,
}

#[derive(Clone)]
pub(super) struct HamburgerButtonStyle {
    pub size: f32,
    pub normal: UiImage,
    pub hover: UiImage,
    pub press: UiImage,
}

#[derive(Clone)]
pub(super) struct TextStyle {
    pub font: FontHandle,
    pub font_size: f32,
    pub color: [f32; 4],
}

#[derive(Clone)]
pub(super) struct MenuStyle {
    pub background: UiImage,
}
