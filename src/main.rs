use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
};

mod systems;

mod pong;
use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    // RenderToWindow provides for opening and drawing in a window
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]), // background
                )
                // RenderFlat2D is used to render entities with a SpriteRenderer component
                .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(
            systems::BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        );

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();

    Ok(())
}