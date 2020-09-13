//! Holds UI styles

use amethyst::assets::{AssetStorage, Loader};
use amethyst::prelude::*;
use amethyst::ui::{get_default_font, FontAsset, FontHandle, UiImage};

/// Loads the UI style and places it in the world.
pub fn load_style(world: &mut World) {
    let white = [1.0, 1.0, 1.0, 1.0];
    let black = [0.0, 0.0, 0.0, 0.0];

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
        font: font.clone(),
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
        normal: UiImage::SolidColor(black),
        hover: UiImage::SolidColor([0.1, 0.1, 0.1, 1.0]),
        press: UiImage::SolidColor([0.15, 0.15, 0.15, 1.0]),
    };

    let ui_style = Style {
        button,
        title_text,
        menu,
        paragraph: label.clone(),
        label: label.clone(),
    };

    world.insert(ui_style);
}

#[derive(Clone)]
struct Style {
    button: ButtonStyle,
    title_text: TextStyle,
    menu: MenuStyle,
    paragraph: TextStyle,
    label: TextStyle,
}

#[derive(Clone)]
struct ButtonStyle {
    text: TextStyle,
    width: f32,
    height: f32,
    normal: UiImage,
    hover: UiImage,
    press: UiImage,
}

#[derive(Clone)]
struct TextStyle {
    font: FontHandle,
    font_size: f32,
    color: [f32; 4],
}

#[derive(Clone)]
struct MenuStyle {
    background: UiImage,
}
