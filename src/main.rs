mod events;
mod states;

use amethyst::{
    core::transform::TransformBundle, input, prelude::*, renderer, ui, utils::application_root_dir,
};

use structopt::StructOpt;

fn main() -> amethyst::Result<()> {
    let _args = CliArgs::from_args();
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets/");
    let config_dir = app_root.join("config/");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            input::InputBundle::<events::InputBindingTypes>::new()
                .with_bindings_from_file(config_dir.join("input.ron"))?,
        )?
        .with_bundle(ui::UiBundle::<events::InputBindingTypes>::new())?
        .with_bundle(
            renderer::RenderingBundle::<renderer::types::DefaultBackend>::new()
                .with_plugin(
                    renderer::RenderToWindow::from_config_path(config_dir.join("display.ron"))?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(ui::RenderUi::default())
                .with_plugin(renderer::RenderFlat2D::default()),
        )?;

    let mut game = CoreApplication::<_, events::StateEvent, events::StateEventReader>::build(
        assets_dir,
        states::Loading,
    )?
    .build(game_data)?;

    game.run();
    Ok(())
}

/// Free and open-source tic-tac-toe.
///
/// For information on how to play FossXO, select *Help* from the game's
/// main menu.
#[derive(StructOpt, Debug)]
struct CliArgs {}

#[cfg(test)]
mod tests {

    #[test]
    fn has_tests() {
        assert!(true)
    }
}
