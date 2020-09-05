// Only safe code is allowed in this application.
#![forbid(unsafe_code)]
// On Windows do not show a console window.
// Note: this also suppresses output from the unit tests.
#![windows_subsystem = "windows"]

mod components;
mod constants;
mod environments;
mod events;
mod file_io;
mod math;
mod resources;
mod states;
mod systems;

use amethyst::{
    core::frame_limiter, core::transform::TransformBundle, input, prelude::*, renderer, ui, window,
};

use structopt::StructOpt;

fn main() -> amethyst::Result<()> {
    let _args = CliArgs::from_args();
    amethyst::start_logger(Default::default());
    log::info!("Started FossXO v{}.", constants::FOSSXO_VERSION);
    log::info!("Operating system: {}", os_info::get());

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            input::InputBundle::<events::InputBindingTypes>::new()
                .with_bindings_from_file(file_io::input_bindings_file()?)?,
        )?
        .with_bundle(ui::UiBundle::<events::InputBindingTypes>::new())?
        .with_bundle(systems::GameBundle)?
        .with_bundle(systems::EnvironmentsBundle)?
        .with_bundle(
            renderer::RenderingBundle::<renderer::types::DefaultBackend>::new()
                .with_plugin(
                    renderer::RenderToWindow::from_config(display_configuration())
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(ui::RenderUi::default())
                .with_plugin(renderer::RenderFlat2D::default())
                .with_plugin(renderer::RenderDebugLines::default()),
        )?;

    let mut game = CoreApplication::<_, events::StateEvent, events::StateEventReader>::build(
        file_io::assets_dir()?,
        states::Loading,
    )?
    .with_frame_limit(frame_limiter::FrameRateLimitStrategy::Sleep, 60)
    .build(game_data)?;

    game.run();

    log::info!("Thanks for playing FossXO!");
    Ok(())
}

fn display_configuration() -> window::DisplayConfig {
    let mut config = window::DisplayConfig::default();
    config.title = "FossXO".to_string();
    config.dimensions = Some((800, 600));
    config.min_dimensions = Some((800, 600));
    config
}

/// Free and open-source tic-tac-toe.
///
/// For information on how to play FossXO, select *Help* from the game's
/// main menu.
#[derive(StructOpt, Debug)]
struct CliArgs {}
