use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::WindowMode;
use bigmoe::GamePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Windowed,
                position: WindowPosition::Centered(MonitorSelection::Index(2)),
                resolution: Vec2::new(1920.0, 1080.0).into(),
                title: "minor moe".to_string(), // ToDo
                resizable: false,               // does not stop programmatic resize
                decorations: true,              // close, min, max, etc.
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup_camera.run_if(run_once()))
        .run();

    println!("Hello, BigMode!");
}

fn setup_camera(mut commands: Commands, query_window: Query<&Window>) {
    let window = query_window.single();
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                viewport: Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(window.width() as u32, window.height() as u32),
                    depth: Default::default(),
                }),
                order: 0,
                is_active: true,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
            },
            ..default()
        },
        UiCameraConfig { show_ui: true },
    ));
    info!("Spawned Camera");
}
