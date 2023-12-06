use bevy::prelude::*;
use bigmoe::GamePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: bevy::window::WindowMode::Windowed,
                        position: WindowPosition::Centered(MonitorSelection::Index(2)),
                        resolution: Vec2::new(1920.0, 1080.0).into(),
                        title: "minor moe".to_string(), // ToDo
                        resizable: false,               // does not stop programmatic resize
                        decorations: true,              // close, min, max, etc.
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(GamePlugin)
        .run();

    println!("Hello, BigMode!");
}
