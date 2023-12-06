pub mod ian;
pub mod loading;
pub mod menu;
pub mod metronome;
pub mod moe;

use crate::ian::IanPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::metronome::MetronomePlugin;
use crate::moe::MoePlugin;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy_framepace::FramepacePlugin;
use std::time::Duration;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<SongState>()
            .add_plugins((
                LoadingPlugin,
                MenuPlugin,
                MoePlugin,
                IanPlugin,
                FramepacePlugin,
                MetronomePlugin,
            ))
            .add_systems(
                OnEnter(GameState::Branding),
                (setup_camera, spawn_branding).run_if(run_once()),
            )
            .add_systems(
                Update,
                (tick_transition, transition_to_loading).run_if(in_state(GameState::Branding)),
            );
    }
}

#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub enum GameState {
    #[default]
    Branding,
    Loading,
    Menu,
    Playing,
}
#[derive(States, Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub enum SongState {
    #[default]
    Stopped,
    Playing,
    Paused,
}

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct AnimationData {
    pub timer: Timer,
    pub frames: usize,
    pub count: usize,
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
fn spawn_branding(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/brand.png"),
            transform: Transform::from_scale(Vec3::new(10., 10., 0.)),
            ..default()
        },
        BrandingTransition {
            timer: Timer::new(Duration::from_secs_f32(3.0), TimerMode::Once),
        },
    ));
}
fn tick_transition(mut query_transition: Query<&mut BrandingTransition>, time: Res<Time>) {
    if let Ok(mut transition) = query_transition.get_single_mut() {
        transition.timer.tick(time.delta());
    }
}
fn transition_to_loading(
    mut commands: Commands,
    query_transition: Query<(Entity, &BrandingTransition)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, transition) in query_transition.iter() {
        if transition.timer.finished() {
            commands.entity(entity).despawn_recursive();
            next_state.set(GameState::Loading);
        }
    }
}

#[derive(Component)]
struct BrandingTransition {
    timer: Timer,
}

// constants
pub const SCREENRATIO: UVec2 = UVec2 { x: 288, y: 162 };
pub const PALETTEMAIN: &str = "palette/main.png";
